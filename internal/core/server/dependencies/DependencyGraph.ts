/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {SourceLocation} from "@internal/parser-core";
import {
	AnalyzeModuleType,
	BundleBuddyStats,
	ServerRequest,
} from "@internal/core";
import {DiagnosticsProcessor, catchDiagnostics} from "@internal/diagnostics";
import {ResolverOptions} from "../fs/Resolver";
import WorkerQueue from "../WorkerQueue";
import DependencyNode from "./DependencyNode";
import {ReporterProgress} from "@internal/cli-reporter";
import {Locker} from "../../../async/lockers";
import {DependencyOrder} from "./DependencyOrderer";
import {WorkerAnalyzeDependencyResult} from "../../common/bridges/WorkerBridge";

import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createUnknownPath,
} from "@internal/path";

import {markup} from "@internal/markup";
import {FileNotFound, MissingFileReturn} from "@internal/fs/FileNotFound";

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
	"worker_threads",
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
	}

	private request: ServerRequest;
	private resolverOpts: ResolverOptions;
	private server: Server;
	private nodes: AbsoluteFilePathMap<DependencyNode>;
	private locker: Locker<string>;

	public getNodes(): Iterable<DependencyNode> {
		return this.nodes.values();
	}

	private isExternal(path: AbsoluteFilePath, source: string): boolean {
		const project = this.server.projectManager.assertProjectExisting(path);
		return (
			project.config.bundler.externals.includes(source) ||
			NODE_BUILTINS.includes(source)
		);
	}

	public getBundleBuddyStats(entries: Array<AbsoluteFilePath>): BundleBuddyStats {
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

	public deleteNode(path: AbsoluteFilePath) {
		this.nodes.delete(path);
	}

	private addNode(path: AbsoluteFilePath, res: WorkerAnalyzeDependencyResult) {
		const module = new DependencyNode(
			this.server,
			this,
			this.server.projectManager.getFileReference(path),
			res,
		);
		this.nodes.set(path, module);
		return module;
	}

	public maybeGetNode(path: AbsoluteFilePath): undefined | DependencyNode {
		return this.nodes.get(path);
	}

	public getNode(path: AbsoluteFilePath): DependencyNode {
		const mod = this.maybeGetNode(path);
		if (mod === undefined) {
			throw new FileNotFound(path, "No dependency node found");
		}
		return mod;
	}

	public async seed(
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
		// Initialize sub dependency queue
		const workerQueue: DependencyGraphWorkerQueue = new WorkerQueue(
			this.server,
			{
				callback: async ({path, item}) => {
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
				},
			},
		);
		await workerQueue.prepare(paths);

		// Initialize roots
		const rootQueue: WorkerQueue<void> = new WorkerQueue(
			this.server,
			{
				callback: async ({path}) => {
					const ret = await FileNotFound.maybeAllowMissing(
						allowFileNotFound,
						path,
						() => {
							return this.resolve(
								path,
								{
									workerQueue,
									all: true,
									async: false,
									ancestry: [],
								},
								diagnosticsProcessor,
								analyzeProgress,
							);
						},
					);
					roots.push(ret);
				},
			},
		);
		const roots: Array<MissingFileReturn<DependencyNode>> = [];

		for (const path of paths) {
			await FileNotFound.maybeAllowMissing(
				allowFileNotFound,
				path,
				() => rootQueue.pushPath(path),
			);
		}

		// Spin root queue
		await rootQueue.spin();

		// Spin worker queue
		await workerQueue.spin();

		if (diagnosticsProcessor.hasDiagnostics()) {
			return;
		}

		if (validate) {
			for (const ret of roots) {
				if (!ret.missing) {
					const root = ret.value;
					await FileNotFound.maybeAllowMissing(
						allowFileNotFound,
						root.path,
						() => this.validateTransitive(root, diagnosticsProcessor),
					);
				}
			}
		}
	}

	public validate(
		node: DependencyNode,
		diagnosticsProcessor: DiagnosticsProcessor,
	): boolean {
		const resolvedImports = node.resolveImports();
		return (
			diagnosticsProcessor.addDiagnostics(resolvedImports.diagnostics).length >
			0
		);
	}

	private validateTransitive(
		node: DependencyNode,
		diagnosticsProcessor: DiagnosticsProcessor,
	) {
		const order = node.getDependencyOrder();
		diagnosticsProcessor.addDiagnostics(order.diagnostics);

		for (const path of order.files) {
			this.validate(this.getNode(path), diagnosticsProcessor);
		}
	}

	private async resolve(
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
		let progressId;

		if (analyzeProgress !== undefined) {
			progressId = analyzeProgress.pushText(progressText);
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
								source: createUnknownPath(source),
							},
							dep.loc === undefined
								? undefined
								: {
										location: {
											sourceText: undefined,
											...dep.loc,
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
			await opts.workerQueue.pushPath(
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

		if (analyzeProgress !== undefined && progressId !== undefined) {
			analyzeProgress.popText(progressId);
			analyzeProgress.tick();
		}

		return node;
	}
}
