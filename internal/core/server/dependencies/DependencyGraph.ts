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
	WorkerAnalyzeDependencyResult,
} from "@internal/core";
import {DiagnosticsProcessor, catchDiagnostics} from "@internal/diagnostics";
import {ResolverOptions} from "../fs/Resolver";
import WorkerQueue from "../WorkerQueue";
import DependencyNode from "./DependencyNode";
import {ReporterProgress} from "@internal/cli-reporter";
import {Locker} from "../../../async/lockers";
import {DependencyOrder} from "./DependencyOrderer";

import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createPath,
} from "@internal/path";
import {markup} from "@internal/markup";
import FileNotFound, {MissingFileReturn} from "@internal/fs/FileNotFound";
import { areAnalyzeDependencyResultsEqual } from "@internal/compiler";

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
	ancestry: string[];
	type: AnalyzeModuleType;
	loc: undefined | SourceLocation;
	shallowUsedExports: undefined | Set<string>;
};

export type DependencyGraphWorkerQueue = WorkerQueue<SeedQueueItem>;

export type DependencyGraphOptions = {
	shallow?: boolean;
};

export default class DependencyGraph {
	constructor(
		request: ServerRequest,
		resolverOpts: ResolverOptions,
		graphOpts: DependencyGraphOptions = {},
	) {
		this.request = request;
		this.server = request.server;
		this.nodes = new AbsoluteFilePathMap();
		this.graphOpts = graphOpts;
		this.resolverOpts = resolverOpts;
		this.locker = new Locker();
	}

	private request: ServerRequest;
	private resolverOpts: ResolverOptions;
	private graphOpts: DependencyGraphOptions;
	private server: Server;
	private nodes: AbsoluteFilePathMap<DependencyNode>;
	private locker: Locker<string>;

	public close() {
		
	}

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

	public getBundleBuddyStats(entries: AbsoluteFilePath[]): BundleBuddyStats {
		const stats: BundleBuddyStats = [];

		for (const node of this.nodes.values()) {
			const source = node.uid.join();

			for (const absoluteTarget of node.relativeToAbsolutePath.values()) {
				const target = this.getNode(absoluteTarget).uid.join();
				stats.push({
					target,
					source,
				});
			}
		}

		for (const absoluteEntry of entries) {
			const source = this.getNode(absoluteEntry).uid.join();
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

	public hasNode(path: AbsoluteFilePath): boolean {
		return this.nodes.has(path);
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
			paths: AbsoluteFilePath[];
			diagnosticsProcessor: DiagnosticsProcessor;
			analyzeProgress?: ReporterProgress;
			allowFileNotFound?: boolean;
			validate?: boolean;
		},
	): Promise<void> {
		// Initialize sub dependency queue
		const workerQueue: DependencyGraphWorkerQueue = this.server.createWorkerQueue({
			callback: async ({path, item}) => {
				await this.resolve(
					path,
					{
						workerQueue,
						all: item.all,
						async: item.async,
						ancestry: item.ancestry,
						shallowUsedExports: item.shallowUsedExports,
						isRoot: false,
					},
					diagnosticsProcessor,
					analyzeProgress,
				);
			},
		});
		await workerQueue.prepare(paths);

		// Initialize roots
		const rootQueue = this.server.createWorkerQueue({
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
								isRoot: true,
							},
							diagnosticsProcessor,
							analyzeProgress,
						);
					},
				);
				roots.push(ret);
			},
		});
		const roots: MissingFileReturn<DependencyNode>[] = [];

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
	): void {
		const resolvedImports = node.resolveImports();
		diagnosticsProcessor.addDiagnostics(resolvedImports.diagnostics);
	}

	public async evictNodes(paths: AbsoluteFilePathSet, reseed: (paths: AbsoluteFilePathSet, dependents: boolean) => Promise<void>): Promise<AbsoluteFilePathSet> {
		// Get all the current dependency nodes for the evicted files, and invalidate their nodes
		const oldEvictedNodes: AbsoluteFilePathMap<DependencyNode> = new AbsoluteFilePathMap();
		for (const path of paths) {
			const node = this.maybeGetNode(path);
			if (node !== undefined) {
				oldEvictedNodes.set(path, node);
				this.deleteNode(path);
			}
		}

		// Refresh only the evicted paths
		await reseed(paths, false);

		// Maintain a list of all the dependencies we revalidated
		const validatedDependencyPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		// Maintain a list of all the dependents that need to be revalidated
		const validatedDependencyPathDependents: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		// Build a list of dependents to recheck
		for (const path of paths) {
			const newNode = this.maybeGetNode(path);
			if (newNode === undefined) {
				continue;
			}

			validatedDependencyPaths.add(path);

			// Get the previous node and see if the exports have actually changed
			const oldNode = oldEvictedNodes.get(path);
			const sameShape =
				oldNode !== undefined &&
				areAnalyzeDependencyResultsEqual(
					oldNode.analyze.value,
					newNode.analyze.value,
				);

			for (const depNode of newNode.getDependents()) {
				// If the old node has the same shape as the new one, only revalidate the dependent if it had dependency errors
				// NB: We might want to revalidate if it depended on an evictedPath that was deleted
				if (
					sameShape &&
					depNode.hadResolveImportsDiagnostics === false
				) {
					continue;
				}

				validatedDependencyPaths.add(depNode.path);
				validatedDependencyPathDependents.add(depNode.path);
				this.deleteNode(depNode.path);
			}
		}

		// Revalidate dependents
		if (validatedDependencyPathDependents.size > 0) {
			await reseed(validatedDependencyPathDependents, true);
		}

		return validatedDependencyPaths;
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
			ancestry: string[];
			workerQueue: DependencyGraphWorkerQueue;
			isRoot: boolean;
			shallowUsedExports?: Set<string>;
		},
		diagnosticsProcessor: DiagnosticsProcessor,
		analyzeProgress?: ReporterProgress,
	): Promise<DependencyNode> {
		const filename = path.join();
		const {async, all, ancestry} = opts;
		const {server} = this;

		let res: undefined | WorkerAnalyzeDependencyResult;
		let node: undefined | DependencyNode;

		// We have a lock here in case we hit `this.resolve` while we're waiting for the `analyzeDependencies` result
		const lock = await this.locker.getLock(filename);

		if (this.nodes.has(path)) {
			node = this.getNode(path);
			res = node.analyze;
			node.setAll(all);
			node.setUsedAsync(async);

			// If this node is shallow then it is incomplete and we haven't checked all of it's dependencies
			// So recalculate it
			if (!node.shallow) {
				lock.release();
				return node;
			}
		}

		const progressText = markup`<filelink target="${filename}" />`;
		let progressId;

		if (analyzeProgress !== undefined) {
			progressId = analyzeProgress.pushText(progressText);
		}

		try {
			if (res === undefined) {
				res = await this.request.requestWorkerAnalyzeDependencies(path, {});
			}

			if (node === undefined) {
				node = this.addNode(path, res);
			}

			node.setAll(all);
			node.setUsedAsync(async);
		} finally {
			lock.release();
		}

		let {dependencies, diagnostics, exports} = res.value;

		if (diagnostics.length > 0) {
			diagnosticsProcessor.addDiagnostics(diagnostics);
		}

		// If we're a remote path then the origin should be the URL and not our local path
		const remote = this.server.projectManager.getRemoteFromLocalPath(path);
		const origin = remote === undefined ? path : remote.getParent();

		let isShallowNode = false;

		dependencies = dependencies.filter((dep) => {
			const {source} = dep;

			if (this.isExternal(path, source)) {
				return false;
			}

			// If we've been given a shallowUsedExports list then only include this dependency if it could include that export
			const {shallowUsedExports} = opts;
			if (shallowUsedExports !== undefined) {
				if (!dep.exported) {
					isShallowNode = true;
					return false;
				}

				for (const exp of exports) {
					if (exp.type === "local") {
						continue;
					}

					if (exp.source !== source) {
						continue;
					}

					if (exp.type === "externalAll") {
						return true;
					}

					if (exp.type === "external" && shallowUsedExports.has(exp.exported)) {
						return true;
					}
				}

				isShallowNode = true;
				return false;
			}

			return true;
		});

		node.setShallow(isShallowNode);

		// Resolve full locations
		await Promise.all(
			dependencies.map(async (dep) => {
				const {source, optional} = dep;

				const {diagnostics} = await catchDiagnostics(
					async () => {
						const resolved = await server.resolver.resolveAssert(
							{
								...this.resolverOpts,
								origin,
								source: createPath(source),
								location: dep.loc === undefined
									? undefined
									: {
										...dep.loc,
										sourceText: undefined,
										integrity: undefined,
									},
							},
						);

						node!.addDependency(source, resolved.path, dep);
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
			const dep = node.getDependencyInfoFromAbsolute(path);

			let shallowUsedExports: undefined | Set<string>;
			if (this.graphOpts.shallow) {
				// Get all the export names that we use from this module
				shallowUsedExports = new Set(dep.names.map((elem) => elem.name));
			}

			await opts.workerQueue.pushPath(
				path,
				{
					all: dep.all,
					async: dep.async,
					type: dep.type,
					loc: dep.loc,
					ancestry: subAncestry,
					shallowUsedExports,
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
