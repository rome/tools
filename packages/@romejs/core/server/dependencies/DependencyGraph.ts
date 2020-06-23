/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {SourceLocation} from "@romejs/parser-core";
import {BundleBuddyStats} from "../../common/types/bundler";
import {DiagnosticsProcessor, catchDiagnostics} from "@romejs/diagnostics";
import {ResolverOptions} from "../fs/Resolver";
import WorkerQueue from "../WorkerQueue";
import DependencyNode from "./DependencyNode";
import {ReporterProgress} from "@romejs/cli-reporter";
import {Locker} from "../../common/utils/locker";
import {DependencyOrder} from "./DependencyOrderer";
import {Event} from "@romejs/events";
import {WorkerAnalyzeDependencyResult} from "../../common/bridges/WorkerBridge";
import {ServerRequest} from "@romejs/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createUnknownFilePath,
} from "@romejs/path";
import {AnalyzeModuleType} from "../../common/types/analyzeDependencies";
import {markup} from "@romejs/string-markup";
import {FileNotFound} from "@romejs/core/common/FileNotFound";

export type DependencyGraphSeedResult = {
	node: DependencyNode;
	order: DependencyOrder;
	cached: boolean;
};

const NODE_BUILTINS = [
	"electron",
	"buffer",
	"child_process",
	"crypto",
	"dgram",
	"dns",
	"fs",
	"http",
	"https",
	"net",
	"os",
	"readline",
	"stream",
	"string_decoder",
	"tls",
	"tty",
	"zlib",
	"constants",
	"events",
	"url",
	"assert",
	"util",
	"path",
	"punycode",
	"querystring",
	"cluster",
	"console",
	"module",
	"process",
	"vm",
	"domain",
	"v8",
	"repl",
	"timers",
	"inspector",
];

type SeedQueueItem = {
	all: boolean;
	async: boolean;
	ancestry: Array<string>;
	type: AnalyzeModuleType;
	loc: undefined | SourceLocation;
};

export type DependencyGraphWorkerQueue = WorkerQueue<SeedQueueItem>;

export default class DependencyGraph {
	constructor(request: ServerRequest, resolverOpts: ResolverOptions) {
		this.request = request;
		this.server = request.server;
		this.nodes = new AbsoluteFilePathMap();
		this.resolverOpts = resolverOpts;

		this.locker = new Locker();
		this.closeEvent = new Event({name: "DependencyGraph.closeEvent"});
	}

	request: ServerRequest;
	resolverOpts: ResolverOptions;
	server: Server;
	nodes: AbsoluteFilePathMap<DependencyNode>;
	locker: Locker<string>;
	closeEvent: Event<void, void>;

	close() {
		this.closeEvent.send();
	}

	isExternal(path: AbsoluteFilePath, source: string): boolean {
		const project = this.server.projectManager.assertProjectExisting(path);
		return (
			project.config.bundler.externals.includes(source) ||
			NODE_BUILTINS.includes(source)
		);
	}

	getBundleBuddyStats(entries: Array<AbsoluteFilePath>): BundleBuddyStats {
		const stats: BundleBuddyStats = [];

		for (const node of this.nodes.values()) {
			const source = node.uid;

			for (const absoluteTarget of node.relativeToAbsolutePath.values()) {
				const target = this.getNode(absoluteTarget).uid;
				stats.push({
					target,
					source,
				});
			}
		}

		for (const absoluteEntry of entries) {
			const source = this.getNode(absoluteEntry).uid;
			stats.push({
				source,
				target: undefined,
			});
		}

		return stats;
	}

	deleteNode(path: AbsoluteFilePath) {
		this.nodes.delete(path);
	}

	addNode(path: AbsoluteFilePath, res: WorkerAnalyzeDependencyResult) {
		const module = new DependencyNode(
			this,
			this.server.projectManager.getFileReference(path),
			res,
		);
		this.nodes.set(path, module);
		return module;
	}

	maybeGetNode(path: AbsoluteFilePath): undefined | DependencyNode {
		return this.nodes.get(path);
	}

	getNode(path: AbsoluteFilePath): DependencyNode {
		const mod = this.maybeGetNode(path);
		if (mod === undefined) {
			throw new FileNotFound(path, "No dependency node found");
		}
		return mod;
	}

	async seed(
		{
			paths,
			diagnosticsProcessor,
			analyzeProgress,
			allowFileNotFound = false,
			validate = false,
		}: {
			paths: Array<AbsoluteFilePath>;
			diagnosticsProcessor: DiagnosticsProcessor;
			analyzeProgress?: ReporterProgress;
			allowFileNotFound?: boolean;
			validate?: boolean;
		},
	): Promise<void> {
		const workerQueue: DependencyGraphWorkerQueue = new WorkerQueue(this.server);

		workerQueue.addCallback(async (path, item) => {
			await this.resolve(
				path,
				{
					workerQueue,
					all: item.all,
					async: item.async,
					ancestry: item.ancestry,
				},
				diagnosticsProcessor,
				analyzeProgress,
			);
		});

		// Add initial queue items
		const roots: Array<undefined | DependencyNode> = await Promise.all(
			paths.map((path) =>
				FileNotFound.maybeAllowMissing(
					allowFileNotFound,
					path,
					() =>
						this.resolve(
							path,
							{
								workerQueue,
								all: true,
								async: false,
								ancestry: [],
							},
							diagnosticsProcessor,
							analyzeProgress,
						)
					,
				)
			),
		);

		await workerQueue.spin();

		if (diagnosticsProcessor.hasDiagnostics()) {
			return;
		}

		if (validate) {
			for (const root of roots) {
				if (root !== undefined) {
					await FileNotFound.maybeAllowMissing(
						allowFileNotFound,
						root.path,
						() => this.validateTransitive(root, diagnosticsProcessor),
					);
				}
			}
		}
	}

	validate(
		node: DependencyNode,
		diagnosticsProcessor: DiagnosticsProcessor,
	): boolean {
		const resolvedImports = node.resolveImports();
		return (
			diagnosticsProcessor.addDiagnostics(resolvedImports.diagnostics).length >
			0
		);
	}

	validateTransitive(
		node: DependencyNode,
		diagnosticsProcessor: DiagnosticsProcessor,
	) {
		const order = node.getDependencyOrder();
		diagnosticsProcessor.addDiagnostics(order.diagnostics);

		for (const path of order.files) {
			this.validate(this.getNode(path), diagnosticsProcessor);
		}
	}

	async resolve(
		path: AbsoluteFilePath,
		opts: {
			all: boolean;
			async: boolean;
			ancestry: Array<string>;
			workerQueue: DependencyGraphWorkerQueue;
		},
		diagnosticsProcessor: DiagnosticsProcessor,
		analyzeProgress?: ReporterProgress,
	): Promise<DependencyNode> {
		const filename = path.join();
		const {async, all, ancestry} = opts;
		const {server} = this;

		// We have a lock here in case we hit `this.resolve` while we're waiting for the `analyzeDependencies` result
		const lock = await this.locker.getLock(filename);

		if (this.nodes.has(path)) {
			const node = this.getNode(path);

			if (all) {
				node.setAll(true);
			}

			if (async) {
				node.setUsedAsync(true);
			}

			lock.release();

			return node;
		}

		const progressText = markup`<filelink target="${filename}" />`;

		if (analyzeProgress !== undefined) {
			analyzeProgress.pushText(progressText);
		}

		let res: WorkerAnalyzeDependencyResult;
		let node: DependencyNode;
		try {
			res = await this.request.requestWorkerAnalyzeDependencies(path, {});

			node = this.addNode(path, res);
			node.setAll(all);
			node.setUsedAsync(async);
		} finally {
			lock.release();
		}

		const {dependencies, diagnostics} = res;

		if (diagnostics.length > 0) {
			diagnosticsProcessor.addDiagnostics(diagnostics);
		}

		// If we're a remote path then the origin should be the URL and not our local path
		const remote = this.server.projectManager.getRemoteFromLocalPath(path);
		const origin = remote === undefined ? path : remote.getParent();

		// Resolve full locations
		await Promise.all(
			dependencies.map(async (dep) => {
				const {source, optional} = dep;
				if (this.isExternal(path, source)) {
					return;
				}

				const {diagnostics} = await catchDiagnostics(
					async () => {
						const resolved = await server.resolver.resolveAssert(
							{
								...this.resolverOpts,
								origin,
								source: createUnknownFilePath(source),
							},
							dep.loc === undefined
								? undefined
								: {
										location: {
											sourceText: undefined,
											...dep.loc,
											language: "js",
											mtime: undefined,
										},
									},
						);

						node.addDependency(source, resolved.path, dep);
					},
					{
						category: "DependencyGraph",
						message: "Caught by resolve",
					},
				);

				if (diagnostics !== undefined && !optional) {
					diagnosticsProcessor.addDiagnostics(diagnostics);
				}
			}),
		);

		// Queue our dependencies...
		const subAncestry = [...ancestry, filename];
		for (const path of node.getAbsoluteDependencies()) {
			const dep = node.getDependencyInfoFromAbsolute(path).analyze;
			await opts.workerQueue.pushQueue(
				path,
				{
					all: dep.all,
					async: dep.async,
					type: dep.type,
					loc: dep.loc,
					ancestry: subAncestry,
				},
			);
		}

		if (analyzeProgress !== undefined) {
			analyzeProgress.popText(progressText);
			analyzeProgress.tick();
		}

		return node;
	}
}
