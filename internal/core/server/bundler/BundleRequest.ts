/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Bundler from "./Bundler";
import DependencyNode from "../dependencies/DependencyNode";
import {Mappings, SourceMapGenerator} from "@internal/codec-source-map";
import {BundleRequestResult, BundlerMode} from "../../common/types/bundler";
import {
	WorkerBundleCompileOptions,
	WorkerCompileResult,
} from "../../common/bridges/WorkerBridge";
import {DependencyOrder} from "../dependencies/DependencyOrderer";
import {
	BundleCompileResolvedImports,
	CompileResult,
	getPrefixedBundleNamespace,
} from "@internal/compiler";

import {DiagnosticsProcessor, descriptions} from "@internal/diagnostics";
import {AbsoluteFilePath} from "@internal/path";
import {ob1Add} from "@internal/ob1";
import {readFile} from "@internal/fs";
import crypto = require("crypto");

import {Dict} from "@internal/typescript-helpers";
import {Reporter} from "@internal/cli-reporter";
import WorkerQueue from "../WorkerQueue";
import {dedent} from "@internal/string-utils";
import {markup} from "@internal/markup";
import DependencyGraph from "../dependencies/DependencyGraph";
import {Server, ServerRequest} from "@internal/core";

export type BundleOptions = {
	prefix?: string;
	interpreter?: string;
	deferredSourceMaps?: boolean;
};

export default class BundleRequest {
	constructor(
		{
			bundler,
			reporter,
			graph,
			request,
			mode,
			server,
			resolvedEntry,
			options,
		}: {
			bundler: Bundler;
			request: ServerRequest;
			graph: DependencyGraph;
			reporter: Reporter;
			mode: BundlerMode;
			resolvedEntry: AbsoluteFilePath;
			options: BundleOptions;
			server: Server;
		},
	) {
		this.options = options;
		this.request = request;
		this.reporter = reporter;
		this.bundler = bundler;
		this.graph = graph;
		this.server = server;
		this.cached = true;
		this.mode = mode;

		this.resolvedEntry = resolvedEntry;
		this.resolvedEntryUid = server.projectManager.getUid(resolvedEntry);

		this.diagnostics = request.createDiagnosticsProcessor({
			origins: [
				{
					category: "bundler",
					message: `Requested bundle for <filelink target="${this.resolvedEntryUid}" />`,
				},
			],
		});
		this.diagnostics.addAllowedUnusedSuppressionPrefix("lint");

		this.compiles = new Map();
		this.assets = new Map();

		this.sourceMap = new SourceMapGenerator({
			file: resolvedEntry.getBasename(),
		});
	}

	public diagnostics: DiagnosticsProcessor;

	private request: ServerRequest;
	private graph: DependencyGraph;
	private server: Server;
	private options: BundleOptions;
	private cached: boolean;
	private reporter: Reporter;
	private bundler: Bundler;
	private resolvedEntry: AbsoluteFilePath;
	private resolvedEntryUid: string;
	private assets: Map<string, Buffer>;
	private compiles: Map<string, CompileResult>;
	private sourceMap: SourceMapGenerator;
	private mode: BundlerMode;

	public async stepAnalyze(): Promise<DependencyOrder> {
		const {reporter, graph} = this;

		const analyzeProgress = reporter.progress({
			name: `bundler:analyze:${this.resolvedEntryUid}`,
			title: markup`Analyzing`,
		});
		this.diagnostics.setThrowAfter(100);
		try {
			await graph.seed({
				paths: [this.resolvedEntry],
				diagnosticsProcessor: this.diagnostics,
				analyzeProgress,
				validate: true,
			});
		} finally {
			analyzeProgress.end();
		}

		return graph.getNode(this.resolvedEntry).getDependencyOrder();
	}

	private async stepCompile(paths: Array<AbsoluteFilePath>) {
		const {server} = this;
		const {reporter} = this;
		this.diagnostics.setThrowAfter(undefined);

		const compilingSpinner = reporter.progress({
			name: `bundler:compile:${this.resolvedEntryUid}`,
			title: markup`Compiling`,
		});
		compilingSpinner.setTotal(paths.length);

		const queue: WorkerQueue<void> = new WorkerQueue(server);

		queue.addCallback(async (path) => {
			const progressId = compilingSpinner.pushText(markup`${path}`);
			await this.compileJS(path);
			compilingSpinner.popText(progressId);
			compilingSpinner.tick();
		});

		for (const path of paths) {
			await queue.pushQueue(path);
		}

		await queue.spin();
		compilingSpinner.end();
	}

	public async compileJS(path: AbsoluteFilePath): Promise<WorkerCompileResult> {
		const {graph} = this;

		const source = path.join();
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

		let assetPath: undefined | string;
		if (mod.handler?.isAsset) {
			const buffer = await readFile(mod.path);

			// Asset path in the form of: BASENAME-SHA1HASH.EXTENSIONS
			const hash = crypto.createHash("sha1").update(buffer).digest("hex");
			const basename = mod.path.getExtensionlessBasename();
			const exts = mod.path.getExtensions();

			assetPath = `${basename}-${hash}${exts}`;
			this.assets.set(assetPath, buffer);
		}

		const opts: WorkerBundleCompileOptions = {
			mode: this.mode,
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

		if (!res.cached) {
			this.cached = false;
		}

		this.diagnostics.addSuppressions(res.suppressions);
		this.diagnostics.addDiagnostics(res.diagnostics);

		this.compiles.set(source, res);
		return res;
	}

	private stepCombine(
		order: DependencyOrder,
		forceSourceMaps: boolean,
	): BundleRequestResult {
		const {files} = order;
		const {inlineSourceMap} = this.bundler.config;
		const {resolvedEntry, mode, sourceMap, graph} = this;

		// We allow deferring the generation of source maps. We don't do this by default as it's slower than generating them upfront
		// which is what most callers need. But for things like tests, we want to lazily compute the source map only when diagnostics
		// are present.
		let deferredSourceMaps =
			!forceSourceMaps && this.options.deferredSourceMaps === true;
		if (deferredSourceMaps) {
			sourceMap.addMaterializer(() => {
				this.stepCombine(order, true);
			});
		}

		let content: string = "";
		let lineOffset: number = 0;

		function push(str: string) {
			str += "\n";
			content += str;
			if (!deferredSourceMaps) {
				for (let cha of str) {
					if (cha === "\n") {
						lineOffset++;
					}
				}
			}
		}

		function addMappings(
			filename: string,
			sourceContent: string,
			mappings: Mappings,
		) {
			if (deferredSourceMaps) {
				return;
			}

			sourceMap.setSourceContent(filename, sourceContent);
			for (const mapping of mappings) {
				sourceMap.addMapping({
					...mapping,
					generated: {
						...mapping.generated,
						line: ob1Add(lineOffset, mapping.generated.line),
					},
				});
			}
		}

		const {interpreter} = this.options;
		if (interpreter !== undefined) {
			push(`#!${interpreter}\n`);
		}

		push(
			dedent`
			(function(res) {
				if (typeof module !== "undefined") {
					module.exports = res;
				}
				return res;
			})(`,
		);

		// add on bootstrap
		if (order.firstTopAwaitLocations.length > 0) {
			if (mode === "legacy") {
				for (const {loc, mtime} of order.firstTopAwaitLocations) {
					this.diagnostics.addDiagnostic({
						description: descriptions.BUNDLER.TOP_LEVEL_AWAIT_IN_LEGACY,
						location: {
							...loc,
							mtime,
						},
					});
				}
			}

			push("(async function(global) {");
		} else {
			push("(function(global) {");
		}

		if (mode === "modern") {
			push(`  'use strict';`);
		}

		// TODO prelude

		/*
    const path = createAbsoluteFilePath(loc);
    const res = await this.bundler.request.requestWorkerCompile(
      path,
      'compile',
    );
    push('(function() {');
    addMappings(
      this.bundler.server.projectManager.getUid(path),
      res.src,
      res.mappings,
    );
    push(res.code);
    push('})();');
    */
		const declaredCJS: Set<DependencyNode> = new Set();
		function declareCJS(module: DependencyNode) {
			if (mode !== "modern" || module.type !== "cjs" || declaredCJS.has(module)) {
				return;
			}

			declaredCJS.add(module);

			push(`  var ${getPrefixedBundleNamespace(module.uid)} = {};`);
		}

		// Add on files
		for (const source of files) {
			const module = graph.getNode(source);

			for (const path of module.getAbsoluteDependencies()) {
				declareCJS(graph.getNode(path));
			}

			const compileResult = this.compiles.get(source.join());
			if (compileResult === undefined) {
				continue;
				throw new Error("Expected compile result");
			}

			// Only do this in modern mode, the module id will already be in the wrapper otherwise
			if (mode === "modern") {
				push(`  // ${module.uid}`);
			}

			declareCJS(module);

			addMappings(module.uid, compileResult.sourceText, compileResult.mappings);
			push(compileResult.compiledCode);
			push("");
		}

		// push on initial entry require
		const entryModule = graph.getNode(resolvedEntry);
		if (mode === "modern") {
			push(`  return ${getPrefixedBundleNamespace(entryModule.uid)};`);
		} else {
			push(`  return Rome.requireNamespace("${entryModule.uid}");`);
		}

		// push footer
		push(
			"})(typeof global !== 'undefined' ? global : typeof window !== 'undefined' ? window : this));",
		);

		//
		if (inlineSourceMap) {
			const sourceMapComment = sourceMap.toComment();
			content += sourceMapComment;
		} else {
			content += `//# sourceMappingURL=${this.sourceMap.file}.map`;
		}

		return {
			diagnostics: this.diagnostics.getDiagnostics(),
			content,
			sourceMap: this.sourceMap,
			cached: this.cached,
			assets: this.assets,
		};
	}

	private shouldAbort(): boolean {
		return this.diagnostics.hasDiagnostics();
	}

	private abort(): BundleRequestResult {
		return {
			sourceMap: this.sourceMap,
			content: "",
			diagnostics: this.diagnostics.getDiagnostics(),
			cached: false,
			assets: this.assets,
		};
	}

	public async bundle(): Promise<BundleRequestResult> {
		const order = await this.stepAnalyze();
		if (this.shouldAbort()) {
			return this.abort();
		}

		// Compile
		await this.stepCompile(order.files);
		if (this.shouldAbort()) {
			return this.abort();
		}

		// Combine
		return this.stepCombine(order, false);
	}
}
