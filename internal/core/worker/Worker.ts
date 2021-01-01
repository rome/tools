/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature, TypeCheckProvider} from "@internal/js-analysis";
import WorkerBridge, {
	PrefetchedModuleSignatures,
	WorkerBufferPatch,
	WorkerParseOptions,
	WorkerPartialManifest,
	WorkerPartialManifests,
	WorkerProjects,
} from "../common/bridges/WorkerBridge";
import {AnyRoot, ConstJSSourceType, JSRoot} from "@internal/ast";
import Logger from "../common/utils/Logger";
import {Profiler} from "@internal/v8";
import {UserConfig} from "@internal/core";
import {DiagnosticIntegrity, DiagnosticsError} from "@internal/diagnostics";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UnknownPathMap,
	createAbsoluteFilePath,
	createUnknownPath,
} from "@internal/path";
import {
	FSReadStream,
	FSStats,
	createFakeStats,
	createReadStream,
} from "@internal/fs";
import {FileReference} from "../common/types/files";
import {getFileHandlerFromPathAssert} from "../common/file-handlers/index";
import {TransformProjectDefinition} from "@internal/compiler";
import WorkerAPI from "./WorkerAPI";
import {applyWorkerBufferPatch} from "./utils/applyWorkerBufferPatch";
import VirtualModules from "../common/VirtualModules";
import {markup} from "@internal/markup";
import {BridgeClient, BridgeError} from "@internal/events";
import {ExtendedMap} from "@internal/collections";
import WorkerCache from "./WorkerCache";
import FatalErrorHandler from "../common/FatalErrorHandler";
import {RSERObject} from "@internal/codec-binary-serial";

export type ParseResult = {
	ast: AnyRoot;
	integrity: undefined | DiagnosticIntegrity;
	mtimeNs: bigint;
	project: TransformProjectDefinition;
	path: AbsoluteFilePath;
	lastAccessed: number;
	sourceText: string;
	astModifiedFromSource: boolean;
};

export type WorkerBuffer = {
	content: string;
	mtimeNs: bigint;
};

type WorkerOptions = {
	userConfig: UserConfig;
	dedicated: boolean;
	bridge: BridgeClient<typeof WorkerBridge>;
	id: number;
};

export default class Worker {
	constructor(opts: WorkerOptions) {
		this.bridge = opts.bridge;

		this.userConfig = opts.userConfig;
		this.partialManifests = new ExtendedMap("partialManifests");
		this.projects = new Map();
		this.astCache = new AbsoluteFilePathMap();
		this.moduleSignatureCache = new UnknownPathMap();
		this.buffers = new AbsoluteFilePathMap();
		this.virtualModules = new VirtualModules();

		this.logger = new Logger(
			{},
			{
				loggerType: "worker",
				check: () => opts.bridge.events.log.hasSubscribers(),
				write(chunk) {
					opts.bridge.events.log.send(chunk.toString());
				},
			},
		);
		opts.bridge.updatedListenersEvent.subscribe(() => {
			this.logger.updateStream();
		});

		this.cache = new WorkerCache(this);
		this.api = new WorkerAPI(this, this.logger, this.cache);

		this.fatalErrorHandler = new FatalErrorHandler({
			getOptions: (err) => {
				try {
					const {bridge} = this;

					// Dispatch error to the server and trigger a fatal
					bridge.events.fatalError.send(bridge.serializeError(err));
				} catch (err) {
					if (!(err instanceof BridgeError)) {
						console.error(
							"Worker encountered error while attempting to send a fatal to the server",
						);
						console.error(err.stack);
					}
					process.exit(1);
				}
				return false;
			},
		});

		if (opts.dedicated) {
			this.fatalErrorHandler.setupGlobalHandlers();

			// Pretty sure we'll hit another error condition before this but for completeness
			/*opts.bridge.monitorHeartbeat(
				LAG_INTERVAL,
				({iterations, totalTime}) => {
					if (iterations >= 5) {
						console.error(`Server has not responded for ${totalTime}ms. Exiting.`)
						process.exit(1);
					}
				},
			);*/
		}
	}

	public userConfig: UserConfig;
	public api: WorkerAPI;
	public logger: Logger;
	public fatalErrorHandler: FatalErrorHandler;
	public virtualModules: VirtualModules;

	private cache: WorkerCache;
	private bridge: BridgeClient<typeof WorkerBridge>;
	private partialManifests: ExtendedMap<number, WorkerPartialManifest>;
	private projects: Map<number, TransformProjectDefinition>;
	private astCache: AbsoluteFilePathMap<ParseResult>;
	private moduleSignatureCache: UnknownPathMap<ModuleSignature>;
	private buffers: AbsoluteFilePathMap<WorkerBuffer>;

	public getPartialManifest(id: number): WorkerPartialManifest {
		return this.partialManifests.assert(id);
	}

	private async end() {
		this.astCache.clear();
		this.projects.clear();
		this.moduleSignatureCache.clear();
		await this.cache.teardown();
	}

	public async init() {
		this.virtualModules.init();

		const bridge: BridgeClient<typeof WorkerBridge> = this.bridge;

		bridge.endEvent.subscribe(async () => {
			await this.end();
		});

		let profiler: undefined | Profiler;
		bridge.events.profilingStart.subscribe(async (data) => {
			if (profiler !== undefined) {
				throw new Error("Expected no profiler to be running");
			}
			profiler = new Profiler();
			await profiler.startProfiling(data.samplingInterval);
		});

		bridge.events.profilingStop.subscribe(async () => {
			if (profiler === undefined) {
				throw new Error("Expected a profiler to be running");
			}
			const workerProfile = await profiler.stopProfiling();
			profiler = undefined;
			return workerProfile;
		});

		bridge.events.compile.subscribe((payload) => {
			return this.api.compile(
				payload.ref,
				payload.stage,
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.events.parse.subscribe((payload) => {
			// @ts-ignore: AST is a bunch of interfaces which we cannot match with an object index
			return this.api.parse(payload.ref, payload.options) as RSERObject;
		});

		bridge.events.lint.subscribe((payload) => {
			return this.api.lint(payload.ref, payload.options, payload.parseOptions);
		});

		bridge.events.format.subscribe((payload) => {
			return this.api.format(payload.ref, payload.options, payload.parseOptions);
		});

		bridge.events.updateInlineSnapshots.subscribe((payload) => {
			return this.api.updateInlineSnapshots(
				payload.ref,
				payload.updates,
				payload.parseOptions,
			);
		});

		bridge.events.analyzeDependencies.subscribe((payload) => {
			return this.api.analyzeDependencies(payload.ref, payload.parseOptions);
		});

		bridge.events.evict.subscribe(async (payload) => {
			await this.evict(payload);
			return undefined;
		});

		bridge.events.moduleSignatureJS.subscribe((payload) => {
			return this.api.moduleSignatureJS(payload.ref, payload.parseOptions);
		});

		bridge.events.updateProjects.subscribe((payload) => {
			return this.updateProjects(payload.projects);
		});

		bridge.events.updateManifests.subscribe((payload) => {
			return this.updateManifests(payload.manifests);
		});

		bridge.events.status.subscribe(() => {
			return {
				astCacheSize: this.astCache.size,
				pid: process.pid,
				memoryUsage: process.memoryUsage(),
				uptime: process.uptime(),
			};
		});

		bridge.events.getBuffer.subscribe((payload) => {
			return this.getBuffer(payload.ref);
		});

		bridge.events.updateBuffer.subscribe(async (payload) => {
			return this.updateBuffer(payload.ref, payload.buffer);
		});

		bridge.events.patchBuffer.subscribe(async (payload) => {
			return this.patchBuffer(payload.ref, payload.patches);
		});

		bridge.events.clearBuffer.subscribe(async (payload) => {
			return this.clearBuffer(payload.ref);
		});

		bridge.events.getFileBuffers.subscribe(() => {
			return this.getFileBuffers();
		});
	}

	public isDiskSynced(path: AbsoluteFilePath): boolean {
		return !this.buffers.has(path) && !this.virtualModules.isVirtualPath(path);
	}

	public hasBuffer(path: AbsoluteFilePath): boolean {
		return this.buffers.has(path);
	}

	public getBufferFakeStats(path: AbsoluteFilePath): FSStats {
		const buffer = this.buffers.assert(path);
		return createFakeStats({
			type: "file",
			size: BigInt(buffer.content.length),
			date: new Date(Number(buffer.mtimeNs / 1000000n)),
		});
	}

	public getBuffer(ref: FileReference): undefined | string {
		this.logger.info(markup`Returned ${ref.real} buffer`);
		const buffer = this.buffers.get(ref.real);
		if (buffer === undefined) {
			return undefined;
		} else {
			return buffer.content;
		}
	}

	public async clearBuffer(ref: FileReference) {
		this.logger.info(markup`Cleared ${ref.real} buffer`);
		this.buffers.delete(ref.real);
		await this.evict(ref);
	}

	public async updateBuffer(ref: FileReference, buffer: WorkerBuffer) {
		this.logger.info(markup`Updated ${ref.real} buffer`);
		this.buffers.set(ref.real, buffer);
		await this.evict(ref);
	}

	private getFileBuffers(): [AbsoluteFilePath, WorkerBuffer][] {
		return Array.from(this.buffers);
	}

	private async patchBuffer(ref: FileReference, patches: WorkerBufferPatch[]) {
		this.logger.info(markup`Patched ${ref.real} buffer`);
		const entry = this.buffers.assert(ref.real);
		const {mtimeNs: mtime} = entry;
		let buffer: undefined | string = entry.content;

		// Patches must be applied sequentially
		for (const patch of patches) {
			buffer = applyWorkerBufferPatch(buffer, patch);

			if (buffer === undefined) {
				throw new Error(`Invalid patch for buffer of ${ref.real.join()}`);
			}
		}

		this.buffers.set(ref.real, {content: buffer, mtimeNs: mtime});
		await this.evict(ref);
		return buffer;
	}

	public async getTypeCheckProvider(
		projectId: number,
		prefetchedModuleSignatures: PrefetchedModuleSignatures = {},
		parseOptions: WorkerParseOptions,
	): Promise<TypeCheckProvider> {
		const libs: JSRoot[] = [];

		// TODO Figure out how to get the uids for the libraries, probably adding some additional stuff to ProjectConfig?

		/*
    const projectConfig = this.getProjectConfig(projectId);
    for (const filename of projectConfig.typeChecking.libs) {
      const {ast, err} = await this.parse(filename, uid, projectId);
      if (err) {
        throw err;
      } else {
        invariant(ast, 'expected ast');
        libs.push(ast);
      }
    }
    */
		const resolveGraph = async (
			key: string,
		): Promise<undefined | ModuleSignature> => {
			const value = prefetchedModuleSignatures[key];
			if (value === undefined) {
				return undefined;
			}

			switch (value.type) {
				case "RESOLVED": {
					this.moduleSignatureCache.set(
						createUnknownPath(value.graph.filename),
						value.graph,
					);
					return value.graph;
				}

				case "OWNED":
					return this.api.moduleSignatureJS(value.ref, parseOptions);

				case "POINTER":
					return resolveGraph(value.key);

				case "USE_CACHED": {
					return this.moduleSignatureCache.assert(
						createUnknownPath(value.filename),
					);
				}
			}
		};

		return {
			getExportTypes: async (
				origin: string,
				relative: string,
			): Promise<undefined | ModuleSignature> => {
				return resolveGraph(`${origin}:${relative}`);
			},
			libs,
		};
	}

	public async readFileText(ref: FileReference): Promise<string> {
		const content = await this.readFile(ref);

		if (typeof content === "string") {
			return content;
		} else {
			return new Promise((resolve, reject) => {
				let buff = "";

				content.on(
					"error",
					(err) => {
						reject(err);
					},
				);

				content.on(
					"data",
					(chunk) => {
						buff += chunk;
					},
				);

				content.on(
					"end",
					() => {
						resolve(buff);
					},
				);
			});
		}
	}

	public async readFile(ref: FileReference): Promise<string | FSReadStream> {
		const buffer = this.buffers.get(ref.real);
		if (buffer !== undefined) {
			return buffer.content;
		}

		const virtual = this.virtualModules.getPossibleVirtualFileContents(ref.real);
		if (virtual !== undefined) {
			return virtual;
		}

		// We may have already read this file to hash it for the cache
		const cache = await this.cache.getFile(ref);
		const cached = cache.takePossibleReadFile();
		if (cached !== undefined) {
			return cached;
		}

		return createReadStream(ref.real);
	}

	public async parse(
		ref: FileReference,
		options: WorkerParseOptions,
	): Promise<ParseResult> {
		const path = createAbsoluteFilePath(ref.real);

		const {project: projectId, uid} = ref;
		const project = this.getProject(projectId);

		// Fetch and validate extension handler
		const {handler} = getFileHandlerFromPathAssert(ref.real, project.config);
		if (handler.parse === undefined) {
			throw new Error(`We don't know how to parse ${path}`);
		}

		// Get source type
		let sourceTypeJS: undefined | ConstJSSourceType;
		if (options.sourceTypeJS !== undefined) {
			sourceTypeJS = options.sourceTypeJS;
		} else if (handler.sourceTypeJS !== undefined) {
			sourceTypeJS = handler.sourceTypeJS;
		} else {
			sourceTypeJS = "script";

			if (ref.manifest !== undefined) {
				const manifest = this.getPartialManifest(ref.manifest);
				if (manifest.type === "module") {
					sourceTypeJS = "module";
				}
			}
		}

		const cacheEnabled = options.cache !== false;

		if (cacheEnabled) {
			// Update the lastAccessed of the ast cache and return it, it will be evicted on
			// any file change
			const cachedResult: undefined | ParseResult = this.astCache.get(path);
			if (cachedResult !== undefined) {
				let useCached = true;

				if (
					cachedResult.ast.type === "JSRoot" &&
					cachedResult.ast.sourceType !== sourceTypeJS
				) {
					useCached = false;
				}

				if (useCached) {
					this.astCache.set(
						path,
						{
							...cachedResult,
							lastAccessed: Date.now(),
						},
					);
					return cachedResult;
				}
			}
		}

		this.logger.info(markup`Parsing: ${path}`);

		const cacheFile = await this.cache.getFile(ref);
		const integrity = await this.cache.getIntegrity(ref);
		const {mtimeNs} = await cacheFile.getStats();

		let manifestPath: undefined | string;
		if (ref.manifest !== undefined) {
			manifestPath = this.getPartialManifest(ref.manifest).path.join();
		}

		const {sourceText, astModifiedFromSource, ast} = await handler.parse({
			sourceTypeJS,
			path: createUnknownPath(uid),
			manifestPath,
			integrity,
			mtimeNs,
			file: ref,
			worker: this,
			project,
			parseOptions: options,
		});

		// If the AST is corrupt then we don't under any circumstance allow it
		if (ast.corrupt && !options.allowCorrupt) {
			throw new DiagnosticsError("Corrupt AST", ast.diagnostics);
		}

		// Sometimes we may want to allow the "fixed" AST
		if (
			!(options.allowParserDiagnostics || options.allowCorrupt) &&
			ast.diagnostics.length > 0
		) {
			throw new DiagnosticsError(
				"AST diagnostics aren't allowed",
				ast.diagnostics,
			);
		}

		const res: ParseResult = {
			ast,
			lastAccessed: Date.now(),
			sourceText,
			project,
			path,
			astModifiedFromSource,
			integrity,
			mtimeNs,
		};

		if (cacheEnabled) {
			this.astCache.set(path, res);
		}

		return res;
	}

	public getProject(id: number): TransformProjectDefinition {
		const config = this.projects.get(id);
		if (config === undefined) {
			throw new Error(
				`Unknown project ${id}, known projects are ${this.projects.keys()}`,
			);
		}
		return config;
	}

	private async evict(
		{real, uid}: {
			real: AbsoluteFilePath;
			uid: string;
		},
	) {
		this.logger.info(markup`Evicted ${real}`);
		this.astCache.delete(real);
		this.moduleSignatureCache.delete(real);
		await this.cache.remove(uid, real);
	}

	private updateManifests(manifests: WorkerPartialManifests) {
		for (const {id, manifest} of manifests) {
			if (manifest === undefined) {
				this.partialManifests.delete(id);
			} else {
				this.partialManifests.set(id, manifest);
			}
		}
	}

	public updateProjects(projects: WorkerProjects) {
		for (const {config, directory, configHashes, id} of projects) {
			if (config === undefined) {
				this.projects.delete(id);
			} else {
				this.projects.set(
					id,
					{
						directory,
						configHashes,
						config,
					},
				);
			}
		}
	}
}
