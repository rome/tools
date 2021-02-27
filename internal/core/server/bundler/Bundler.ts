/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AssembledBundle,
	Server,
	ServerRequest,
	WorkerBundleCompileOptions,
	WorkerCompileResult,
} from "@internal/core";
import {Reporter} from "@internal/cli-reporter";
import {
	BundleResult,
	BundleResultBundle,
	BundlerConfig,
	BundlerFiles,
	BundleCompileResult,
	BundlerEntryResolution,
	BundleWatcher,
	BundleWatcherFiles,
} from "../../common/types/bundler";
import DependencyGraph from "../dependencies/DependencyGraph";
import BundleRequest, {BundleOptions} from "./BundleRequest";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UIDPath,
	createAnyPath,
	createRelativePath,
	RelativePathMap,
	RelativePathSet,
	RelativePath,
	AnyPath,
	AbsoluteFilePathSet,
} from "@internal/path";
import {
	JSONManifest,
	ManifestDefinition,
	convertManifestToJSON,
} from "@internal/codec-js-manifest";
import {Dict} from "@internal/typescript-helpers";
import {flipPathPatterns} from "@internal/path-match";
import {json} from "@internal/codec-config";
import {markup} from "@internal/markup";
import {BundleCompileResolvedImports} from "@internal/compiler";
import {serializeAssembled} from "./utils";
import {Event} from "@internal/events";
import {getDiagnosticsFromError} from "@internal/diagnostics";
import {GlobalLock} from "@internal/async";
import {sha256} from "@internal/string-utils";

export default class Bundler {
	constructor(req: ServerRequest, config: BundlerConfig) {
		this.config = config;
		this.server = req.server;
		this.reporter = req.reporter;
		this.request = req;

		this.compiles = new AbsoluteFilePathMap();
		this.entries = [];
		this.graph = new DependencyGraph(req, config.resolver);
	}

	public config: BundlerConfig;
	public compiles: AbsoluteFilePathMap<BundleCompileResult>;

	private graph: DependencyGraph;
	private server: Server;
	private request: ServerRequest;
	private reporter: Reporter;
	private entries: AbsoluteFilePath[];

	public close() {
		this.graph.close();
	}

	public static createFromServerRequest(req: ServerRequest): Bundler {
		return new Bundler(req, req.getBundlerConfigFromFlags());
	}

	public async getResolvedEntry(
		unresolvedEntry: string,
	): Promise<BundlerEntryResolution> {
		const {cwd} = this.config;

		const res = await this.server.resolver.resolveEntryAssert({
			...this.config.resolver,
			allowPartial: false,
			origin: cwd,
			source: createAnyPath(unresolvedEntry),
		});

		const {server} = this;
		const resolvedEntry = res.path;

		// Now do the same resolver request but with a package
		const manifestRootResolved = server.resolver.resolveLocal({
			...this.config.resolver,
			origin: cwd,
			requestedType: "package",
			source: createAnyPath(unresolvedEntry),
		});
		const manifestRoot: undefined | AbsoluteFilePath =
			manifestRootResolved.type === "FOUND"
				? manifestRootResolved.path
				: undefined;
		let manifestDef;
		if (manifestRoot !== undefined) {
			const def = server.memoryFs.getManifestDefinition(manifestRoot);
			if (def !== undefined) {
				manifestDef = def;
			}
		}

		const project = this.server.projectManager.assertProjectExisting(resolvedEntry);

		return {manifestDef, resolvedEntry, project};
	}

	private createBundleRequest(
		resolvedEntry: AbsoluteFilePath,
		options: BundleOptions,
		reporter: Reporter,
	): BundleRequest {
		this.entries.push(resolvedEntry);
		return new BundleRequest({
			request: this.request,
			bundler: this,
			graph: this.graph,
			server: this.server,
			resolvedEntry,
			options,
			reporter,
		});
	}

	public serializeAssembled(assembled: AssembledBundle): string {
		return serializeAssembled(
			assembled,
			(path) => {
				const compiled = this.compiles.get(path);
				if (compiled === undefined) {
					return undefined;
				} else {
					return compiled.value.compiledCode;
				}
			},
		);
	}

	public async compileJS(path: AbsoluteFilePath): Promise<BundleCompileResult> {
		const existing = this.compiles.get(path);
		if (existing !== undefined) {
			return existing;
		}

		const {graph} = this;
		const mod = graph.getNode(path);

		// Build a map of relative module sources to module id
		const relativeSourcesToModuleId: Map<string, UIDPath> = new Map();
		for (const [relative, absolute] of mod.relativeToAbsolutePath) {
			const moduleId = graph.getNode(absolute).uid;
			relativeSourcesToModuleId.set(relative, moduleId);
		}

		// Diagnostics would have already been added during the initial DependencyGraph.seed
		// We're doing the work of resolving everything again, maybe we should cache it?
		const resolvedImports: BundleCompileResolvedImports = mod.resolveImports().resolved;

		let asset: undefined | BundleCompileResult["asset"];
		let assetPath: undefined | AnyPath;
		if (mod.handler?.isAsset) {
			// TODO: Maybe add option to allow keeping the contents in memory rather than having a double-read
			// TODO: Also investigate reusing worker cache hashes
			
			// Asset path in the form of: BASENAME-HASH.EXTENSIONS
			const hash = await sha256.async(mod.path.createReadStream());
			const basename = mod.path.getExtensionlessBasename();
			const exts = mod.path.getExtensions();

			const relativeAssetPath = createRelativePath(`${basename}-${hash}${exts}`);
			assetPath = this.config.basePath.append(relativeAssetPath);
			asset = {
				path: relativeAssetPath,
				value: {
					etag: hash,
					content: async () => mod.path.createReadStream(),
				},
			};
		}

		const opts: WorkerBundleCompileOptions = {
			moduleAll: mod.all,
			moduleId: mod.uid,
			relativeSourcesToModuleId,
			resolvedImports,
			assetPath,
		};

		const res: WorkerCompileResult = await this.request.requestWorkerCompile(
			path,
			"compileForBundle",
			{
				bundle: opts,
			},
			{},
		);

		const bundleRes: BundleCompileResult = {
			...res,
			asset,
		};
		this.compiles.set(path, bundleRes);
		return bundleRes;
	}

	public async compileSingle(
		path: AbsoluteFilePath,
	): Promise<WorkerCompileResult> {
		const bundleRequest = this.createBundleRequest(path, {}, this.reporter);
		await bundleRequest.stepAnalyze();
		bundleRequest.diagnostics.maybeThrowDiagnosticsError();
		return await this.compileJS(path);
	}

	// This will take multiple entry points and do some magic to make them more efficient to build in parallel
	public async bundleMultiple(
		entries: AbsoluteFilePath[],
		options: BundleOptions = {},
	): Promise<Map<AbsoluteFilePath, BundleResult>> {
		// Clone so we can mess with it
		entries = [...entries];

		// Seed the dependency graph with all the entries at the same time
		const processor = this.request.createDiagnosticsProcessor({
			origins: [
				{
					category: "Bundler",
					message: "Analyzing dependencies for bundleMultiple",
				},
			],
		});
		const entryUIDs = entries.map((entry) =>
			this.server.projectManager.getUID(entry)
		);
		const analyzeProgress = this.reporter.progress({
			name: `bundler:analyze:${entryUIDs.join(",")}`,
			title: markup`Analyzing`,
		});
		processor.setThrowAfter(100);
		await this.graph.seed({
			paths: entries,
			diagnosticsProcessor: processor,
			analyzeProgress,
			validate: false,
		});
		analyzeProgress.end();
		processor.maybeThrowDiagnosticsError();

		// Compile everything at the same time
		const req = this.createBundleRequest(entries[0], {}, this.reporter);
		await req.stepCompile(
			Array.from(this.graph.getNodes(), (node) => node.path),
		);

		// Now actually bundle them
		const map: Map<AbsoluteFilePath, BundleResult> = new Map();

		const progress = this.reporter.progress({title: markup`Bundling`});
		progress.setTotal(entries.length);

		const silentReporter = this.reporter.fork({
			streams: [],
		});

		const promises: Set<Promise<void>> = new Set();

		// Could maybe do some of this in parallel?
		while (entries.length > 0) {
			const entry = entries.shift()!;

			const promise = (async () => {
				const progressId = progress.pushText(entry);

				map.set(entry, await this.bundle(entry, options, silentReporter));
				progress.popText(progressId);
				progress.tick();
			})();
			promise.then(() => {
				promises.delete(promise);
			});
			promises.add(promise);

			if (promises.size > 5) {
				await Promise.race(Array.from(promises));
			}
		}

		await Promise.all(Array.from(promises));

		progress.end();

		return map;
	}

	public bundleManifestWatch(resolution: BundlerEntryResolution): BundleWatcher {
		const refreshSub = this.server.refreshFileEvent.subscribe(async (paths) => {
			const graphPaths = new AbsoluteFilePathSet();
			for (const {path} of paths) {
				if (this.graph.hasNode(path)) {
					this.compiles.delete(path);
					graphPaths.add(path);
				}
			}
			if (graphPaths.size > 0) {
				watcher.changeEvent.send(graphPaths);
				await this.graph.evictNodes(graphPaths, async (paths) => {
					
				});
				run();
			}
		});
		this.request.resources.add(refreshSub);

		const watcher: BundleWatcher = {
			subscription: refreshSub,
			diagnosticsEvent: new Event("diagnosticsEvent"),
			changeEvent: new Event("changeEvent"),
			filesEvent: new Event("filesEvent"),
		};

		const knownFiles: RelativePathSet = new RelativePathSet();
		const runLock: GlobalLock = new GlobalLock();

		const run = async () => {
			try {
				await runLock.wrap(async () => {
					const {files} = await this.bundleManifest(resolution);

					const changes: BundleWatcherFiles = new RelativePathMap();

					// Add deleted files
					for (const path of knownFiles) {
						if (!files.has(path)) {
							changes.set(path, undefined);
						}
					}

					// Add new/updated files
					for (const [path, def] of files) {
						knownFiles.add(path);
						changes.set(path, def);
					}

					watcher.filesEvent.send(changes);
				});
			} catch (err) {
				const diagnostics = getDiagnosticsFromError(err);
				if (diagnostics === undefined) {
					this.request.handleOutboundError(err);
				} else {
					watcher.diagnosticsEvent.send(diagnostics);
				}
			}
		};

		run();

		return watcher;
	}

	public async bundleManifest(
		{resolvedEntry, manifestDef}: BundlerEntryResolution,
	): Promise<{
		files: BundlerFiles,
		bundles: BundleResultBundle[],
		entry: BundleResultBundle,
	}> {
		let bundles: BundleResultBundle[] = [];
		const files: BundlerFiles = new RelativePathMap();

		const createBundle = async (
			resolvedSegment: AbsoluteFilePath,
			options: BundleOptions,
		): Promise<BundleResultBundle> => {
			const bundle = await this.bundle(resolvedSegment, options);
			for (const [path, content] of bundle.files) {
				files.set(path, content);
			}
			bundles = bundles.concat(bundle.bundles);
			return bundle.entry;
		};

		const entryBundle = await createBundle(resolvedEntry, {});

		//
		const bundleBuddyStats = this.graph.getBundleBuddyStats(this.entries);
		files.set(
			createRelativePath("bundlebuddy.json"),
			{
				kind: "stats",
				etag: entryBundle.etag,
				content: async () => json.stringify(bundleBuddyStats),
			},
		);

		// TODO ensure that __dirname is relative to the project root
		if (manifestDef !== undefined) {
			const newManifest = await this.deriveManifest(
				manifestDef,
				entryBundle,
				createBundle,
				(relative, buffer) => {
					if (!files.has(relative)) {
						files.set(
							relative,
							{
								kind: "file",
								etag: "TODO",
								content: async () => buffer,
							},
						);
					}
				},
			);

			// If we have a `files` array then set it to all the newly added files
			// This will have included files already there that we copied
			if (newManifest.files !== undefined) {
				newManifest.files = Array.from(files.keys(), (path) => path.join());
			}

			// Add a package.json with updated values
			files.set(
				createRelativePath("package.json"),
				{
					kind: "manifest",
					etag: "TODO",
					content: async () => json.stringify(newManifest),
				},
			);
		}

		return {
			files,
			bundles,
			entry: entryBundle,
		};
	}

	private async deriveManifest(
		manifestDef: ManifestDefinition,
		entryBundle: BundleResultBundle,
		createBundle: (
			resolvedSegment: AbsoluteFilePath,
			options: BundleOptions,
		) => Promise<BundleResultBundle>,
		addFile: (relative: RelativePath, buffer: Buffer | string) => void,
	): Promise<JSONManifest> {
		// TODO figure out some way to use bundleMultiple here
		const manifest = manifestDef.manifest;

		const newManifest: JSONManifest = {
			...convertManifestToJSON(manifest),
			main: entryBundle.js.path.join(),
		};

		// TODO inherit some manifest properties from project configs
		const project = this.server.projectManager.findLoadedProject(
			manifestDef.directory,
		);
		if (project !== undefined) {
			if (newManifest.name === undefined) {
				newManifest.name = project.config.name;
			}
		}

		// TODO remove dependencies fields, probably?

		// TODO Compile a index.d.ts

		// Copy manifest.files
		if (manifest.files !== undefined) {
			const paths = this.server.memoryFs.glob(
				manifestDef.directory,
				{
					overrideIgnore: flipPathPatterns(manifest.files),
				},
			);

			for (const path of paths) {
				const relative = manifestDef.directory.relativeForce(path);
				const buffer = await path.readFile();
				addFile(relative, buffer);
			}
		}

		// Compile manifest.bin files
		const bin = manifest.bin;
		if (bin !== undefined) {
			const newBin: Dict<string> = {};
			newManifest.bin = newBin;

			const binConsumer = manifestDef.consumer.get("bin");
			const isBinShorthand = typeof binConsumer.asUnknown() === "string";

			for (const [binName, relative] of manifest.bin) {
				const location = (isBinShorthand
					? binConsumer
					: binConsumer.get(binName)).getDiagnosticLocation("inner-value");

				const absolute = await this.server.resolver.resolveAssert(
					{
						...this.config.resolver,
						origin: manifestDef.directory,
						source: createRelativePath(relative).toExplicitRelative(),
						location,
					},
				);

				const res = await createBundle(
					absolute.path,
					{
						prefix: `bin/${binName}`,
						interpreter: "/usr/bin/env node",
					},
				);
				newBin[binName] = res.js.path.join();
			}
		}

		// TODO `{type: "module"}` will always fail since we've produced CJS bundles
		// rome-ignore lint/js/noDelete: future cleanup
		delete newManifest.type;

		// Remove rome project config
		// rome-ignore lint/js/noDelete: future cleanup
		delete newManifest.rome;

		return newManifest;
	}

	public async bundle(
		resolvedEntry: AbsoluteFilePath,
		options: BundleOptions = {},
		reporter: Reporter = this.reporter,
	): Promise<BundleResult> {
		//reporter.info(markup`Bundling <emphasis>${resolvedEntry}</emphasis>`);
		const req = this.createBundleRequest(resolvedEntry, options, reporter);
		const res = await req.bundle();

		const processor = this.request.createDiagnosticsProcessor();
		processor.addDiagnostics(res.diagnostics);
		processor.maybeThrowDiagnosticsError();

		if (res.cached) {
			reporter.warn(markup`Bundle was built completely from cache`);
		}

		const prefix = options.prefix === undefined ? "" : `${options.prefix}/`;
		const jsPath = createRelativePath(`${prefix}index.js`);
		const mapPath = jsPath.addExtension(".map");

		let serialized: undefined | string;
		const serializeAssembled = async () => {
			if (serialized === undefined) {
				serialized = this.serializeAssembled(res.assembled);
			}
			return serialized;
		};

		const files: BundlerFiles = new RelativePathMap();
		files.set(
			jsPath,
			{
				kind: "entry",
				etag: res.etag,
				content: serializeAssembled,
			},
		);

		files.set(
			mapPath,
			{
				kind: "sourcemap",
				etag: res.etag,
				content: async () => res.sourceMap.toJSON(),
			},
		);

		for (const [path, asset] of res.assets) {
			files.set(
				path,
				{
					kind: "asset",
					...asset,
				},
			);
		}

		const bundle: BundleResultBundle = {
			etag: res.etag,
			js: {
				path: jsPath,
				assembled: res.assembled,
				content: serializeAssembled,
			},
			sourceMap: {
				path: mapPath,
				map: res.sourceMap,
			},
		};
		return {
			bundler: this,
			entry: bundle,
			bundles: [bundle],
			files,
		};
	}
}
