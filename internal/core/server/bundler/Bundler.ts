/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AssembledBundle, Server, ServerRequest} from "@internal/core";
import {Reporter} from "@internal/cli-reporter";
import {
	BundleResult,
	BundleResultBundle,
	BundlerConfig,
	BundlerFiles,
} from "../../common/types/bundler";
import DependencyGraph from "../dependencies/DependencyGraph";
import BundleRequest, {BundleOptions} from "./BundleRequest";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createUnknownPath,
} from "@internal/path";
import {
	JSONManifest,
	ManifestDefinition,
	convertManifestToJSON,
} from "@internal/codec-js-manifest";
import {
	WorkerBundleCompileOptions,
	WorkerCompileResult,
} from "../../common/bridges/WorkerBridge";
import {Dict} from "@internal/typescript-helpers";
import {readFile} from "@internal/fs";
import {flipPathPatterns} from "@internal/path-match";
import {stringifyJSON} from "@internal/codec-json";
import {markup} from "@internal/markup";
import {BundleCompileResolvedImports} from "@internal/compiler";
import {serializeAssembled} from "./utils";
import crypto = require("crypto");

export type BundlerEntryResoluton = {
	manifestDef: undefined | ManifestDefinition;
	resolvedEntry: AbsoluteFilePath;
};

type BundleCompileResult = WorkerCompileResult & {
	asset?: {
		path: string;
		buffer: Buffer;
	};
};

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
	private entries: Array<AbsoluteFilePath>;

	public static createFromServerRequest(req: ServerRequest): Bundler {
		return new Bundler(req, req.getBundlerConfigFromFlags());
	}

	public async getResolvedEntry(
		unresolvedEntry: string,
	): Promise<BundlerEntryResoluton> {
		const {cwd} = this.config;

		const res = await this.server.resolver.resolveEntryAssert({
			...this.config.resolver,
			origin: cwd,
			source: createUnknownPath(unresolvedEntry),
		});

		const {server} = this;
		const resolvedEntry = res.path;

		// Now do the same resolver request but with a package
		const manifestRootResolved = server.resolver.resolveLocal({
			...this.config.resolver,
			origin: cwd,
			requestedType: "package",
			source: createUnknownPath(unresolvedEntry),
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

		return {manifestDef, resolvedEntry};
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
					return compiled.compiledCode;
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
		const relativeSourcesToModuleId: Dict<string> = {};
		for (const [relative, absolute] of mod.relativeToAbsolutePath) {
			const moduleId = graph.getNode(absolute).uid;
			relativeSourcesToModuleId[relative] = moduleId;
		}

		// Diagnostics would have already been added during the initial DependencyGraph.seed
		// We're doing the work of resolving everything again, maybe we should cache it?
		const resolvedImports: BundleCompileResolvedImports = mod.resolveImports().resolved;

		let asset: undefined | BundleCompileResult["asset"];
		let assetPath: undefined | string;
		if (mod.handler?.isAsset) {
			const buffer = await readFile(mod.path);

			// Asset path in the form of: BASENAME-SHA1HASH.EXTENSIONS
			const hash = crypto.createHash("sha1").update(buffer).digest("hex");
			const basename = mod.path.getExtensionlessBasename();
			const exts = mod.path.getExtensions();

			assetPath = `${basename}-${hash}${exts}`;
			asset = {
				path: assetPath,
				buffer,
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
		entries: Array<AbsoluteFilePath>,
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
		const entryUids = entries.map((entry) =>
			this.server.projectManager.getUid(entry)
		);
		const analyzeProgress = this.reporter.progress({
			name: `bundler:analyze:${entryUids.join(",")}`,
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
				const progressId = progress.pushText(markup`${entry}`);

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

	public async bundleManifest(
		{resolvedEntry, manifestDef}: BundlerEntryResoluton,
	) {
		let bundles: Array<BundleResultBundle> = [];
		const files: BundlerFiles = new Map();

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
			"bundlebuddy.json",
			{
				kind: "stats",
				content: () => stringifyJSON(bundleBuddyStats),
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
								content: () => buffer,
							},
						);
					}
				},
			);

			// If we have a `files` array then set it to all the newly added files
			// This will have included files already there that we copied
			if (newManifest.files !== undefined) {
				newManifest.files = Array.from(files.keys());
			}

			// Add a package.json with updated values
			files.set(
				"package.json",
				{
					kind: "manifest",
					content: () => stringifyJSON(newManifest),
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
		addFile: (relative: string, buffer: Buffer | string) => void,
	): Promise<JSONManifest> {
		// TODO figure out some way to use bundleMultiple here
		const manifest = manifestDef.manifest;

		const newManifest: JSONManifest = {
			...convertManifestToJSON(manifest),
			main: entryBundle.js.path,
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
				const relative = manifestDef.directory.relative(path).join();
				const buffer = await readFile(path);
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
						source: createUnknownPath(relative).toExplicitRelative(),
					},
					{
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
				newBin[binName] = res.js.path;
			}
		}

		// TODO `{type: "module"}` will always fail since we've produced CJS bundles
		// rome-ignore lint/js/noDelete
		delete newManifest.type;

		// Remove rome project config
		// rome-ignore lint/js/noDelete
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
		const jsPath = `${prefix}index.js`;
		const mapPath = `${jsPath}.map`;

		let serialized: undefined | string;
		const serializeAssembled = () => {
			if (serialized === undefined) {
				serialized = this.serializeAssembled(res.assembled);
			}
			return serialized;
		};

		const files: BundlerFiles = new Map();
		files.set(
			jsPath,
			{
				kind: "entry",
				content: serializeAssembled,
			},
		);

		files.set(
			mapPath,
			{
				kind: "sourcemap",
				content: () => res.sourceMap.toJSON(),
			},
		);

		for (const [relative, buffer] of res.assets) {
			files.set(
				relative,
				{
					kind: "asset",
					content: () => buffer,
				},
			);
		}

		const bundle: BundleResultBundle = {
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
