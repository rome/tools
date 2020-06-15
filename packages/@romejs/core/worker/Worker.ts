/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature, TypeCheckProvider} from "@romejs/js-analysis";
import WorkerBridge, {
	PrefetchedModuleSignatures,
	WorkerParseOptions,
	WorkerPartialManifest,
	WorkerPartialManifests,
	WorkerProjects,
} from "../common/bridges/WorkerBridge";
import {AnyRoot, ConstSourceType, JSRoot} from "@romejs/ast";
import Logger, {PartialLoggerOptions} from "../common/utils/Logger";
import {Profiler} from "@romejs/v8";

import setupGlobalErrorHandlers from "../common/utils/setupGlobalErrorHandlers";
import {UserConfig, loadUserConfig} from "../common/userConfig";
import {hydrateJSONProjectConfig} from "@romejs/project";
import {Diagnostics, DiagnosticsError} from "@romejs/diagnostics";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UnknownFilePathMap,
	createAbsoluteFilePath,
	createUnknownFilePath,
} from "@romejs/path";
import {lstat, readFileText, writeFile} from "@romejs/fs";
import {
	FileReference,
	convertTransportFileReference,
} from "../common/types/files";
import {getFileHandlerAssert} from "../common/file-handlers/index";
import {TransformProjectDefinition} from "@romejs/compiler";
import WorkerAPI from "./WorkerAPI";
import {FileNotFound} from "../common/FileNotFound";

export type ParseResult = {
	ast: AnyRoot;
	project: TransformProjectDefinition;
	path: AbsoluteFilePath;
	lastAccessed: number;
	sourceText: string;
	generated: boolean;
};

type WorkerOptions = {
	loggerOptions?: PartialLoggerOptions;
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

		this.logger = new Logger(
			{
				...opts.loggerOptions,
				type: "worker",
			},
			() => opts.bridge.log.hasSubscribers(),
			{
				streams: [
					{
						type: "all",
						format: "none",
						columns: Infinity,
						unicode: true,
						write(chunk) {
							opts.bridge.log.send(chunk.toString());
						},
					},
				],
			},
		);

		//
		this.api = new WorkerAPI(this);

		if (opts.globalErrorHandlers) {
			setupGlobalErrorHandlers((err) => {
				// TODO
				err;
			});
		}
	}

	userConfig: UserConfig;

	bridge: WorkerBridge;
	api: WorkerAPI;
	logger: Logger;

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
				convertTransportFileReference(payload.file),
				payload.stage,
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.parse.subscribe((payload) => {
			return this.api.parse(
				convertTransportFileReference(payload.file),
				payload.options,
			);
		});

		bridge.lint.subscribe((payload) => {
			return this.api.lint(
				convertTransportFileReference(payload.file),
				payload.options,
				payload.parseOptions,
			);
		});

		bridge.format.subscribe((payload) => {
			return this.api.format(
				convertTransportFileReference(payload.file),
				payload.parseOptions,
			);
		});

		bridge.updateInlineSnapshots.subscribe((payload) => {
			return this.api.updateInlineSnapshots(
				convertTransportFileReference(payload.file),
				payload.updates,
				payload.parseOptions,
			);
		});

		bridge.analyzeDependencies.subscribe((payload) => {
			return this.api.analyzeDependencies(
				convertTransportFileReference(payload.file),
				payload.parseOptions,
			);
		});

		bridge.evict.subscribe((payload) => {
			this.evict(createAbsoluteFilePath(payload.filename));
			return undefined;
		});

		bridge.moduleSignatureJS.subscribe((payload) => {
			return this.api.moduleSignatureJS(
				convertTransportFileReference(payload.file),
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
				convertTransportFileReference(payload.file),
				payload.content,
			);
		});

		bridge.clearBuffer.subscribe((payload) => {
			return this.clearBuffer(convertTransportFileReference(payload.file));
		});
	}

	clearBuffer({real}: FileReference) {
		this.logger.info(`Cleared ${real.toMarkup()} buffer`);
		this.buffers.delete(real);
		this.evict(real);
	}

	updateBuffer(ref: FileReference, content: string) {
		this.logger.info(`Updated ${ref.real.toMarkup()} buffer`);
		this.buffers.set(ref.real, content);
		this.evict(ref.real);
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
						convertTransportFileReference(value.file),
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
			if (buffer === undefined) {
				return await readFileText(path);
			} else {
				return buffer;
			}
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
		const {handler} = getFileHandlerAssert(ref.real, project.config);
		if (handler.parse === undefined) {
			throw new Error(`We don't know how to parse ${path}`);
		}

		// Get source type
		let sourceType: undefined | ConstSourceType;
		if (options.sourceType !== undefined) {
			sourceType = options.sourceType;
		} else if (handler.sourceType !== undefined) {
			sourceType = handler.sourceType;
		} else {
			sourceType = "script";

			if (ref.manifest !== undefined) {
				const manifest = this.getPartialManifest(ref.manifest);
				if (manifest.type === "module") {
					sourceType = "module";
				}
			}
		}

		if (project.config.bundler.mode === "legacy") {
			sourceType = "module";
		}

		const cacheEnabled = options.cache !== false;

		if (cacheEnabled) {
			// Update the lastAccessed of the ast cache and return it, it will be evicted on
			// any file change
			const cachedResult: undefined | ParseResult = this.astCache.get(path);
			if (cachedResult !== undefined) {
				let useCached = true;

				if (cachedResult.ast.sourceType !== sourceType) {
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

		this.logger.info(`Parsing:`, path.toMarkup());

		const stat = await lstat(path);
		let manifestPath: undefined | string;
		if (ref.manifest !== undefined) {
			manifestPath = this.getPartialManifest(ref.manifest).path;
		}

		const {sourceText, generated, ast} = await handler.parse({
			sourceType,
			path: createUnknownFilePath(uid),
			manifestPath,
			stat,
			file: ref,
			worker: this,
			project,
			parseOptions: options,
		});

		// If the AST is corrupt then we don't under any circumstance allow it
		if (ast.corrupt) {
			throw new DiagnosticsError("Corrupt AST", ast.diagnostics);
		}

		// Sometimes we may want to allow the "fixed" AST
		const allowDiagnostics = options.allowParserDiagnostics === true;
		if (!allowDiagnostics && ast.diagnostics.length > 0) {
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
			generated,
		};

		if (cacheEnabled) {
			this.astCache.set(path, res);
		}

		return res;
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

	async writeFile(path: AbsoluteFilePath, content: string): Promise<void> {
		// Write the file out
		await writeFile(path, content);

		// We just wrote the file but the server watcher hasn't had time to notify us
		this.evict(path);
	}

	evict(path: AbsoluteFilePath) {
		this.logger.info(`Evicted ${path.toMarkup()}`);
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
		for (const {config, folder, id} of projects) {
			if (config === undefined) {
				this.projects.delete(id);
			} else {
				this.projects.set(
					id,
					{
						folder: createAbsoluteFilePath(folder),
						config: hydrateJSONProjectConfig(config),
					},
				);
			}
		}
	}
}
