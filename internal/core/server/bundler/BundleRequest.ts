/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Bundler from "./Bundler";
import {Mappings, SourceMapGenerator} from "@internal/codec-source-map";
import {AssembledBundle, BundleAssets, BundleRequestResult} from "../../common/types/bundler";
import {DependencyOrder} from "../dependencies/DependencyOrderer";
import {getPrefixedBundleNamespace} from "@internal/compiler";
import {DiagnosticsProcessor} from "@internal/diagnostics";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	UIDPath,
	createAnyPath,
	RelativePathMap,
} from "@internal/path";
import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import DependencyGraph from "../dependencies/DependencyGraph";
import {Server, ServerRequest} from "@internal/core";
import {dedent} from "@internal/string-utils";
import DependencyNode from "@internal/core/server/dependencies/DependencyNode";
import crypto = require("crypto");

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
			server,
			resolvedEntry,
			options,
		}: {
			bundler: Bundler;
			request: ServerRequest;
			graph: DependencyGraph;
			reporter: Reporter;
			resolvedEntry: AbsoluteFilePath;
			options: BundleOptions;
			server: Server;
		},
	) {
		this.options = options;
		this.reporter = reporter;
		this.bundler = bundler;
		this.graph = graph;
		this.server = server;
		this.cached = true;

		this.resolvedEntry = resolvedEntry;
		this.resolvedEntryUID = server.projectManager.getUID(resolvedEntry);

		this.diagnostics = request.createDiagnosticsProcessor({
			origins: [
				{
					category: "bundler",
					message: `Requested bundle for ${this.resolvedEntryUID}`,
				},
			],
		});
		this.diagnostics.addAllowedUnusedSuppressionPrefix("lint");

		this.sourceMap = new SourceMapGenerator({
			path: createAnyPath(this.resolvedEntry.getBasename()),
		});

		this.assets = new RelativePathMap();
	}

	public diagnostics: DiagnosticsProcessor;

	private sourceMap: SourceMapGenerator;
	private graph: DependencyGraph;
	private server: Server;
	private options: BundleOptions;
	private cached: boolean;
	private reporter: Reporter;
	private bundler: Bundler;
	private resolvedEntry: AbsoluteFilePath;
	private resolvedEntryUID: UIDPath;
	private assets: BundleAssets;

	public async stepAnalyze(): Promise<DependencyOrder> {
		const {reporter, graph} = this;

		const analyzeProgress = reporter.progress({
			name: `bundler:analyze:${this.resolvedEntryUID}`,
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

	public async stepCompile(paths: AbsoluteFilePath[]) {
		const {server} = this;
		const {reporter} = this;
		this.diagnostics.setThrowAfter(undefined);

		const compilingSpinner = reporter.progress({
			name: `bundler:compile:${this.resolvedEntryUID}`,
			title: markup`Compiling`,
		});
		compilingSpinner.setTotal(paths.length);

		const queue = server.createWorkerQueue({
			callback: async ({path}) => {
				const progressId = compilingSpinner.pushText(path);

				const res = await this.bundler.compileJS(path);

				if (res.asset !== undefined) {
					this.assets.set(res.asset.path, res.asset.value);
				}

				if (!res.cached) {
					this.cached = false;
				}

				this.diagnostics.addSuppressions(res.value.suppressions);
				this.diagnostics.addDiagnostics(res.value.diagnostics);

				compilingSpinner.popText(progressId);
				compilingSpinner.tick();
			},
		});

		for (const path of paths) {
			await queue.pushPath(path);
		}

		await queue.spin();
		compilingSpinner.end();
	}

	private stepCombine(
		order: DependencyOrder,
		forceSourceMaps: boolean = false,
	): BundleRequestResult {
		const etagHash = crypto.createHash("sha256");
		const {sourceMap} = this;

		// We allow deferring the generation of source maps. We don't do this by default as it's slower than generating them upfront
		// which is what most callers need. But for things like tests, we want to lazily compute the source map only when diagnostics
		// are present.
		let deferredSourceMaps = !forceSourceMaps && this.options.deferredSourceMaps;
		if (deferredSourceMaps) {
			sourceMap.addMaterializer(() => {
				this.stepCombine(order, true);
			});
		}

		const assembled: AssembledBundle = [];
		let lineOffset: number = 0;

		function track(str: string) {
			etagHash.update(str);
			if (!deferredSourceMaps) {
				lineOffset++;
				for (let cha of str) {
					if (cha === "\n") {
						lineOffset++;
					}
				}
			}
		}

		function push(str: string) {
			assembled.push([0, str]);
			track(str);
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
						line: mapping.generated.line.add(lineOffset),
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
				})(
			`,
		);

		// add on bootstrap
		if (order.firstTopAwaitLocations.length > 0) {
			push("(async function(global) {");
		} else {
			push("(function(global) {");
		}

		push(`  'use strict';`);

		// TODO prelude

		/*
		const path = createAbsoluteFilePath(loc);
		const res = await this.bundler.request.requestWorkerCompile(
			path,
			'compile',
		);
		push('(function() {');
		addMappings(
			this.bundler.server.projectManager.getUID(path),
			res.src,
			res.mappings,
		);
		push(res.code);
		push('})();');
		*/
		const declaredCJS: AbsoluteFilePathSet = new AbsoluteFilePathSet();
		const declareCJS = (node: DependencyNode) => {
			if (node.type !== "cjs" || declaredCJS.has(node.path)) {
				return;
			}

			declaredCJS.add(node.path);

			const uid = this.server.projectManager.getUID(node.path);
			push(`  var ${getPrefixedBundleNamespace(uid)} = {};`);
		};

		// Add on files
		for (const path of order.files) {
			const node = this.graph.getNode(path);
			const uid = this.server.projectManager.getUID(path).join();

			for (const path of node.getAbsoluteDependencies()) {
				declareCJS(this.graph.getNode(path));
			}

			const compileResult = this.bundler.compiles.assert(path).value;

			push(`  // ${uid}`);

			declareCJS(node);

			addMappings(uid, compileResult.sourceText, compileResult.mappings);

			track(compileResult.compiledCode);
			assembled.push([1, path]);
			push("");
		}

		// push on initial entry require
		const entryModuleUID = this.server.projectManager.getUID(this.resolvedEntry);
		push(`  return ${getPrefixedBundleNamespace(entryModuleUID)};`);

		// push footer
		push(
			"})(typeof global !== 'undefined' ? global : typeof window !== 'undefined' ? window : this));",
		);

		//
		if (this.bundler.config.inlineSourceMap) {
			const sourceMapComment = sourceMap.toComment();
			assembled.push([0, sourceMapComment]);
		} else {
			// TODO: URL is wrong. Use bundler.config.basePath
			assembled.push([
				0,
				`//# sourceMappingURL=${this.sourceMap.path.join()}.map`,
			]);
		}

		const etag = etagHash.digest("hex");

		return {
			etag,
			request: this,
			diagnostics: this.diagnostics.getDiagnostics(),
			assembled,
			sourceMap,
			cached: this.cached,
			assets: this.assets,
		};
	}

	private shouldAbort(): boolean {
		return this.diagnostics.hasDiagnostics();
	}

	private abort(): BundleRequestResult {
		return {
			etag: "",
			request: this,
			sourceMap: this.sourceMap,
			assembled: [],
			diagnostics: this.diagnostics.getDiagnostics(),
			cached: false,
			assets: this.assets,
		};
	}

	public async bundle(combine: boolean = true): Promise<BundleRequestResult> {
		const order = await this.stepAnalyze();
		if (this.shouldAbort()) {
			return this.abort();
		}

		// Compile
		await this.stepCompile(order.files);
		if (this.shouldAbort() || !combine) {
			return this.abort();
		}

		// Combine
		return this.stepCombine(order);
	}
}
