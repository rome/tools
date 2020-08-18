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
import setupGlobalErrorHandlers from "../common/utils/setupGlobalErrorHandlers";
import {UserConfig} from "@internal/core";
import {DiagnosticsError} from "@internal/diagnostics";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UnknownPathMap,
	createAbsoluteFilePath,
	createUnknownPath,
} from "@internal/path";
import {lstat, readFileText} from "@internal/fs";
import {FileReference} from "../common/types/files";
import {getFileHandlerFromPathAssert} from "../common/file-handlers/index";
import {TransformProjectDefinition} from "@internal/compiler";
import WorkerAPI from "./WorkerAPI";
import {applyWorkerBufferPatch} from "./utils/applyWorkerBufferPatch";
import VirtualModules from "../common/VirtualModules";
import {markup} from "@internal/markup";
import {BridgeError} from "@internal/events";
import {ExtendedMap} from "@internal/collections";

export type ParseResult = {
	ast: AnyRoot;
	mtime: undefined | number;
	project: TransformProjectDefinition;
	path: AbsoluteFilePath;
	lastAccessed: number;
	sourceText: string;
	astModifiedFromSource: boolean;
};

type WorkerOptions = {
	userConfig: UserConfig;
	dedicated: boolean;
	bridge: WorkerBridge;
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
				check: () => opts.bridge.log.hasSubscribers(),
				write(chunk) {
					opts.bridge.log.send(chunk.toString());
				},
			},
		);
		opts.bridge.updatedListenersEvent.subscribe(() => {
			this.logger.updateStream();
		});

		this.api = new WorkerAPI(this);

		if (opts.dedicated) {
			setupGlobalErrorHandlers((err) => {
				try {
					// Dispatch error to the server and trigger a fatal
					opts.bridge.fatalError.send(opts.bridge.serializeError(err));
				} catch (err) {
					if (!(err instanceof BridgeError)) {
						console.error(
							"Worker encountered error while attempting to send a fatal to the server",
						);
						console.error(err.stack);
					}
					process.exit(1);
				}
			});

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

	private bridge: WorkerBridge;
	private virtualModules: VirtualModules;
	private partialManifests: ExtendedMap<number, WorkerPartialManifest>;
	private projects: Map<number, TransformProjectDefinition>;
	private astCache: AbsoluteFilePathMap<ParseResult>;
	private moduleSignatureCache: UnknownPathMap<ModuleSignature>;
	private buffers: AbsoluteFilePathMap<string>;

	private getPartialManifest(id: number): WorkerPartialManifest {
		return this.partialManifests.assert(id);
	}

	private end() {
		// This will only actually be called when a Worker is created inside of the Server
		// Clear internal maps for memory, in case the Worker instance sticks around
		this.astCache.clear();
		this.projects.clear();
		this.moduleSignatureCache.clear();
	}

	public async init() {
		this.virtualModules.init();

		const bridge: WorkerBridge = this.bridge;

		bridge.endEvent.subscribe(() => {
			this.end();
		});

		let profiler: undefined | Profiler;
		bridge.profilingStart.subscribe(async (data) => {
			if (profiler !== undefined) {
				throw new Error("Expected no profiler to be running");
			}
			profiler = new Profiler();
			await profiler.startProfiling(data.samplingInterval);
		});

		bridge.profilingStop.subscribe(async () => {
			if (profiler === undefined) {
				throw new Error("Expected a profiler to be running");
			}
			const workerProfile = await profiler.stopProfiling();
			profiler = undefined;
			return workerProfile;
		});

		bridge.compile.subscribe((payload) => {
			return this.api.compile(
				payload.ref,
				payload.stage,
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.parse.subscribe((payload) => {
			return this.api.parse(payload.ref, payload.options);
		});

		bridge.lint.subscribe((payload) => {
			return this.api.lint(payload.ref, payload.options, payload.parseOptions);
		});

		bridge.format.subscribe((payload) => {
			return this.api.format(payload.ref, payload.options, payload.parseOptions);
		});

		bridge.updateInlineSnapshots.subscribe((payload) => {
			return this.api.updateInlineSnapshots(
				payload.ref,
				payload.updates,
				payload.parseOptions,
			);
		});

		bridge.analyzeDependencies.subscribe((payload) => {
			return this.api.analyzeDependencies(payload.ref, payload.parseOptions);
		});

		bridge.evict.subscribe((payload) => {
			this.evict(createAbsoluteFilePath(payload.filename));
			return undefined;
		});

		bridge.moduleSignatureJS.subscribe((payload) => {
			return this.api.moduleSignatureJS(payload.ref, payload.parseOptions);
		});

		bridge.updateProjects.subscribe((payload) => {
			return this.updateProjects(payload.projects);
		});

		bridge.updateManifests.subscribe((payload) => {
			return this.updateManifests(payload.manifests);
		});

		bridge.status.subscribe(() => {
			return {
				astCacheSize: this.astCache.size,
				pid: process.pid,
				memoryUsage: process.memoryUsage(),
				uptime: process.uptime(),
			};
		});

		bridge.getBuffer.subscribe((payload) => {
			return this.getBuffer(payload.ref);
		});

		bridge.updateBuffer.subscribe((payload) => {
			return this.updateBuffer(payload.ref, payload.content);
		});

		bridge.patchBuffer.subscribe((payload) => {
			return this.patchBuffer(payload.ref, payload.patches);
		});

		bridge.clearBuffer.subscribe((payload) => {
			return this.clearBuffer(payload.ref);
		});

		bridge.getFileBuffers.subscribe(() => {
			return this.getFileBuffers();
		});
	}

	public getBuffer(ref: FileReference) {
		this.logger.info(markup`Returned ${ref.real} buffer`);
		return this.buffers.get(ref.real);
	}

	public clearBuffer({real}: FileReference) {
		this.logger.info(markup`Cleared ${real} buffer`);
		this.buffers.delete(real);
		this.evict(real);
	}

	public updateBuffer(ref: FileReference, content: string) {
		this.logger.info(markup`Updated ${ref.real} buffer`);
		this.buffers.set(ref.real, content);
		this.evict(ref.real);
	}

	private getFileBuffers() {
		return Array.from(
			this.buffers,
			([path, content]) => ({filename: path.join(), content}),
		);
	}

	private patchBuffer(ref: FileReference, patches: Array<WorkerBufferPatch>) {
		this.logger.info(markup`Patched ${ref.real} buffer`);
		let buffer: undefined | string = this.buffers.assert(ref.real);

		// Patches must be applied sequentially
		for (const patch of patches) {
			buffer = applyWorkerBufferPatch(buffer, patch);

			if (buffer === undefined) {
				throw new Error(`Invalid patch for buffer of ${ref.real.join()}`);
			}
		}

		this.buffers.set(ref.real, buffer);
		this.evict(ref.real);
		return buffer;
	}

	public async getTypeCheckProvider(
		projectId: number,
		prefetchedModuleSignatures: PrefetchedModuleSignatures = {},
		parseOptions: WorkerParseOptions,
	): Promise<TypeCheckProvider> {
		const libs: Array<JSRoot> = [];

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

	public async readFile(path: AbsoluteFilePath): Promise<string> {
		const buffer = this.buffers.get(path);
		if (buffer !== undefined) {
			return buffer;
		}

		const virtual = this.virtualModules.getPossibleVirtualFileContents(path);
		if (virtual !== undefined) {
			return virtual;
		}

		return await readFileText(path);
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

		const mtime = await this.getMtime(path);

		let manifestPath: undefined | string;
		if (ref.manifest !== undefined) {
			manifestPath = this.getPartialManifest(ref.manifest).path.join();
		}

		const {sourceText, astModifiedFromSource, ast} = await handler.parse({
			sourceTypeJS,
			path: createUnknownPath(uid),
			manifestPath,
			mtime,
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
			!options.allowParserDiagnostics &&
			!options.allowCorrupt &&
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
			mtime,
		};

		if (cacheEnabled) {
			this.astCache.set(path, res);
		}

		return res;
	}

	// Get the file mtime to warn about outdated diagnostics
	// If we have a buffer or virtual module for this file then don't set an mtime since our diagnostics
	// explicitly do not match the file system
	public async getMtime(path: AbsoluteFilePath): Promise<undefined | number> {
		if (this.buffers.has(path) || this.virtualModules.isVirtualPath(path)) {
			return undefined;
		} else {
			const stat = await lstat(path);
			return stat.mtimeMs;
		}
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

	private evict(path: AbsoluteFilePath) {
		this.logger.info(markup`Evicted ${path}`);
		this.astCache.delete(path);
		this.moduleSignatureCache.delete(path);
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
		for (const {config, directory, id} of projects) {
			if (config === undefined) {
				this.projects.delete(id);
			} else {
				this.projects.set(
					id,
					{
						directory,
						config,
					},
				);
			}
		}
	}
}
