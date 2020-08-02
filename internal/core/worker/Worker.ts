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
import {UserConfig, loadUserConfig} from "../common/userConfig";
import {hydrateJSONProjectConfig} from "@internal/project";
import {Diagnostics, DiagnosticsError} from "@internal/diagnostics";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UnknownFilePathMap,
	createAbsoluteFilePath,
	createUnknownFilePath,
} from "@internal/path";
import {lstat, readFileText} from "@internal/fs";
import {
	FileReference,
	convertTransportFileReference,
} from "../common/types/files";
import {getFileHandlerFromPathAssert} from "../common/file-handlers/index";
import {TransformProjectDefinition} from "@internal/compiler";
import WorkerAPI from "./WorkerAPI";
import {FileNotFound} from "../common/FileNotFound";
import {applyWorkerBufferPatch} from "./utils/applyWorkerBufferPatch";
import VirtualModules from "../common/VirtualModules";
import {markup} from "@internal/markup";

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
	userConfig?: UserConfig;
	globalErrorHandlers: boolean;
	bridge: WorkerBridge;
};

export default class Worker {
	constructor(opts: WorkerOptions) {
		this.bridge = opts.bridge;

		this.userConfig =
			opts.userConfig === undefined ? loadUserConfig() : opts.userConfig;
		this.partialManifests = new Map();
		this.projects = new Map();
		this.astCache = new AbsoluteFilePathMap();
		this.moduleSignatureCache = new UnknownFilePathMap();
		this.buffers = new AbsoluteFilePathMap();
		this.virtualModules = new VirtualModules();

		this.logger = new Logger(
			"worker",
			{},
			{
				check: () => opts.bridge.log.hasSubscribers(),
				write(chunk) {
					opts.bridge.log.send(chunk.toString());
				},
			},
		);
		opts.bridge.updatedListenersEvent.subscribe(() => {
			this.logger.updateStream();
		});

		//
		this.api = new WorkerAPI(this);

		if (opts.globalErrorHandlers) {
			setupGlobalErrorHandlers((err) => {
				// Dispatch error to the server and trigger a fatal
				opts.bridge.fatalError.send(opts.bridge.serializeError(err));
			});
		}
	}

	userConfig: UserConfig;
	bridge: WorkerBridge;
	api: WorkerAPI;
	logger: Logger;
	virtualModules: VirtualModules;
	partialManifests: Map<number, WorkerPartialManifest>;
	projects: Map<number, TransformProjectDefinition>;
	astCache: AbsoluteFilePathMap<ParseResult>;
	moduleSignatureCache: UnknownFilePathMap<ModuleSignature>;
	buffers: AbsoluteFilePathMap<string>;

	getPartialManifest(id: number): WorkerPartialManifest {
		const manifest = this.partialManifests.get(id);
		if (manifest === undefined) {
			throw new Error(`Requested manifest ${id} but we don't have it`);
		}
		return manifest;
	}

	end() {
		// This will only actually be called when a Worker is created inside of the Server
		// Clear internal maps for memory, in case the Worker instance sticks around
		this.astCache.clear();
		this.projects.clear();
		this.moduleSignatureCache.clear();
	}

	async init() {
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
				convertTransportFileReference(payload.ref),
				payload.stage,
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.parse.subscribe((payload) => {
			return this.api.parse(
				convertTransportFileReference(payload.ref),
				payload.options,
			);
		});

		bridge.lint.subscribe((payload) => {
			return this.api.lint(
				convertTransportFileReference(payload.ref),
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.format.subscribe((payload) => {
			return this.api.format(
				convertTransportFileReference(payload.ref),
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.updateInlineSnapshots.subscribe((payload) => {
			return this.api.updateInlineSnapshots(
				convertTransportFileReference(payload.ref),
				payload.updates,
				payload.parseOptions,
			);
		});

		bridge.analyzeDependencies.subscribe((payload) => {
			return this.api.analyzeDependencies(
				convertTransportFileReference(payload.ref),
				payload.parseOptions,
			);
		});

		bridge.evict.subscribe((payload) => {
			this.evict(createAbsoluteFilePath(payload.filename));
			return undefined;
		});

		bridge.moduleSignatureJS.subscribe((payload) => {
			return this.api.moduleSignatureJS(
				convertTransportFileReference(payload.ref),
				payload.parseOptions,
			);
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

		bridge.updateBuffer.subscribe((payload) => {
			return this.updateBuffer(
				convertTransportFileReference(payload.ref),
				payload.content,
			);
		});

		bridge.patchBuffer.subscribe((payload) => {
			return this.patchBuffer(
				convertTransportFileReference(payload.ref),
				payload.patches,
			);
		});

		bridge.clearBuffer.subscribe((payload) => {
			return this.clearBuffer(convertTransportFileReference(payload.ref));
		});

		bridge.getFileBuffers.subscribe(() => {
			return this.getFileBuffers();
		});
	}

	clearBuffer({real}: FileReference) {
		this.logger.info(markup`Cleared ${real} buffer`);
		this.buffers.delete(real);
		this.evict(real);
	}

	updateBuffer(ref: FileReference, content: string) {
		this.logger.info(markup`Updated ${ref.real} buffer`);
		this.buffers.set(ref.real, content);
		this.evict(ref.real);
	}

	getFileBuffers() {
		return Array.from(
			this.buffers,
			([path, content]) => ({filename: path.join(), content}),
		);
	}

	patchBuffer(ref: FileReference, patches: Array<WorkerBufferPatch>) {
		this.logger.info(markup`Patched ${ref.real} buffer`);
		let buffer = this.buffers.get(ref.real);
		if (buffer === undefined) {
			throw new Error(`Can't find buffer to patch for ${ref.real.join()}`);
		}
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

	async getTypeCheckProvider(
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
						createUnknownFilePath(value.graph.filename),
						value.graph,
					);
					return value.graph;
				}

				case "OWNED":
					return this.api.moduleSignatureJS(
						convertTransportFileReference(value.ref),
						parseOptions,
					);

				case "POINTER":
					return resolveGraph(value.key);

				case "USE_CACHED": {
					const cached = this.moduleSignatureCache.get(
						createUnknownFilePath(value.filename),
					);
					if (cached === undefined) {
						throw new Error(
							`Server told us we have the export types for ${value.filename} cached but we dont!`,
						);
					}
					return cached;
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

	populateDiagnosticsMtime(diagnostics: Diagnostics): Diagnostics {
		return diagnostics;
	}

	async readFile(path: AbsoluteFilePath): Promise<string> {
		try {
			const buffer = this.buffers.get(path);
			if (buffer !== undefined) {
				return buffer;
			}

			const virtual = this.virtualModules.getPossibleVirtualFileContents(path);
			if (virtual !== undefined) {
				return virtual;
			}

			return await readFileText(path);
		} catch (err) {
			if (err.code === "ENOENT") {
				throw new FileNotFound(path, "fs.readFile ENOENT");
			} else {
				throw err;
			}
		}
	}

	async parse(
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

		if (project.config.bundler.mode === "legacy") {
			sourceTypeJS = "module";
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
			manifestPath = this.getPartialManifest(ref.manifest).path;
		}

		const {sourceText, astModifiedFromSource, ast} = await handler.parse({
			sourceTypeJS,
			path: createUnknownFilePath(uid),
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
	async getMtime(path: AbsoluteFilePath): Promise<undefined | number> {
		if (this.buffers.has(path) || this.virtualModules.isVirtualPath(path)) {
			return undefined;
		} else {
			const stat = await lstat(path);
			return stat.mtimeMs;
		}
	}

	getProject(id: number): TransformProjectDefinition {
		const config = this.projects.get(id);
		if (config === undefined) {
			throw new Error(
				`Unknown project ${id}, known projects are ${this.projects.keys()}`,
			);
		}
		return config;
	}

	evict(path: AbsoluteFilePath) {
		this.logger.info(markup`Evicted ${path}`);
		this.astCache.delete(path);
		this.moduleSignatureCache.delete(path);
	}

	updateManifests(manifests: WorkerPartialManifests) {
		for (const {id, manifest} of manifests) {
			if (manifest === undefined) {
				this.partialManifests.delete(id);
			} else {
				this.partialManifests.set(id, manifest);
			}
		}
	}

	updateProjects(projects: WorkerProjects) {
		for (const {config, directory, id} of projects) {
			if (config === undefined) {
				this.projects.delete(id);
			} else {
				this.projects.set(
					id,
					{
						directory: createAbsoluteFilePath(directory),
						config: hydrateJSONProjectConfig(config),
					},
				);
			}
		}
	}
}
