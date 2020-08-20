import {AbsoluteFilePath, AbsoluteFilePathSet} from "@internal/path";
import {
	ProjectConfigCategoriesWithIgnore,
	ProjectDefinition,
} from "@internal/project";
import {Server} from "@internal/core";
import {EventSubscription, mergeEventSubscriptions} from "@internal/events";
import {
	PathPatterns,
	matchPathPatterns,
	parsePathPattern,
} from "@internal/path-match";
import MemoryFileSystem from "@internal/core/server/fs/MemoryFileSystem";
import {GlobalLock} from "@internal/async";

const GLOB_IGNORE: PathPatterns = [parsePathPattern({input: "node_modules"})];

function concatGlobIgnore(patterns: PathPatterns): PathPatterns {
	// If there are any negate patterns then it'll never include GLOB_IGNORE
	for (const pattern of patterns) {
		if (pattern.type === "PathPattern" && pattern.negate) {
			return patterns;
		}
	}

	return [...GLOB_IGNORE, ...patterns];
}

export interface GlobOptions {
	args: Iterable<AbsoluteFilePath>;
	extensions?: Array<string>;
	overrideIgnore?: PathPatterns;
	configCategory?: ProjectConfigCategoriesWithIgnore;
	test?: (path: AbsoluteFilePath) => boolean;
	onWatch?: (sub: EventSubscription) => void;
	onSearchNoMatch?: (path: AbsoluteFilePath) => void;
}
export type WatchFilesEvent = {
	paths: AbsoluteFilePathSet;
	initial: boolean;
};

export type WatchFilesCallback = (opts: WatchFilesEvent) => Promise<void>;

export class Globber {
	constructor(server: Server, opts: GlobOptions) {
		this.opts = opts;
		this.server = server;
		this.memoryFs = server.memoryFs;
		this.ignoresByProject = new WeakMap();
		this.args = new AbsoluteFilePathSet(opts.args);
	}

	private ignoresByProject: WeakMap<ProjectDefinition, PathPatterns>;
	private args: AbsoluteFilePathSet;
	private server: Server;
	private memoryFs: MemoryFileSystem;
	private opts: GlobOptions;

	private getIgnore(path: AbsoluteFilePath): PathPatterns {
		const {configCategory, overrideIgnore} = this.opts;
		const project = this.server.projectManager.findLoadedProject(path);

		let ignore: PathPatterns = overrideIgnore ?? [];
		if (configCategory === undefined || project === undefined) {
			return ignore;
		}

		const projectIgnore = this.ignoresByProject.get(project);
		if (projectIgnore === undefined) {
			ignore = concatGlobIgnore([
				...ignore,
				...project.config[configCategory].ignore,
			]);
			this.ignoresByProject.set(project, ignore);
			return ignore;
		} else {
			return projectIgnore;
		}
	}

	public search(cwd: AbsoluteFilePath): AbsoluteFilePathSet {
		const {extensions, test} = this.opts;
		const {memoryFs} = this;

		const matches: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		let queue: Array<{
			cwd: AbsoluteFilePath;
			path: AbsoluteFilePath;
		}> = [{cwd, path: cwd}];

		while (queue.length > 0) {
			const {path, cwd} = queue.pop()!;

			const ignore = this.getIgnore(path);
			const ignoreMatched = matchPathPatterns(path, ignore, cwd);

			// Don't even recurse into explicit matches
			if (ignoreMatched === "EXPLICIT_MATCH") {
				continue;
			}

			// Add if a matching file
			if (memoryFs.isFile(path) && ignoreMatched === "NO_MATCH") {
				if (test !== undefined && !test(path)) {
					continue;
				}

				// Check extensions
				if (extensions !== undefined) {
					let matchedExt = false;
					for (const ext of extensions) {
						matchedExt = path.hasEndExtension(ext);
						if (matchedExt) {
							break;
						}
					}
					if (!matchedExt) {
						continue;
					}
				}

				matches.add(path);
				continue;
			}

			// Crawl if we're a directory
			// NOTE: We still continue crawling on implicit matches
			if (memoryFs.isDirectory(path)) {
				for (const subpath of memoryFs.readdir(path)) {
					queue.push({cwd, path: subpath});
				}
			}

			// TODO maybe throw? not a file or directory, doesn't exist!
		}

		return matches;
	}

	public async get(safe: boolean = true): Promise<AbsoluteFilePathSet> {
		let paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		for (const arg of this.args) {
			// Make sure project has been initialized
			await this.server.projectManager.findProject(arg);

			const subPaths = this.search(arg);

			if (safe && subPaths.size === 0 && this.opts.onSearchNoMatch) {
				await this.opts.onSearchNoMatch(arg);
			}

			paths = new AbsoluteFilePathSet([...paths, ...subPaths]);
		}

		return paths;
	}

	public async watch(callback: WatchFilesCallback): Promise<EventSubscription> {
		const watcher = new GlobberWatcher(this, this.server, this.args, callback);

		const sub = await watcher.init();

		if (this.opts.onWatch !== undefined) {
			this.opts.onWatch(sub);
		}

		return sub;
	}
}

class GlobberWatcher {
	constructor(
		globber: Globber,
		server: Server,
		args: AbsoluteFilePathSet,
		callback: WatchFilesCallback,
	) {
		this.globber = globber;
		this.server = server;
		this.memoryFs = server.memoryFs;

		this.callback = callback;
		this.args = args;
		this.flushLock = new GlobalLock();

		this.batchPaths = undefined;
	}

	private globber: Globber;
	private args: AbsoluteFilePathSet;
	private server: Server;
	private memoryFs: MemoryFileSystem;

	private batchPaths: undefined | AbsoluteFilePathSet;
	private callback: WatchFilesCallback;
	private flushLock: GlobalLock;

	isDependentPath(path: AbsoluteFilePath): boolean {
		for (const arg of this.args) {
			if (path.equal(arg) || path.isRelativeTo(arg)) {
				return true;
			}
		}
		return false;
	}

	async flushPaths(paths: Array<AbsoluteFilePath>) {
		let pendingPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
		for (const path of paths) {
			if (this.isDependentPath(path)) {
				const paths = this.globber.search(path);
				for (const path of paths) {
					if (this.batchPaths === undefined) {
						pendingPaths.add(path);
					} else {
						this.batchPaths.add(path);
					}
				}
			}
		}
		if (pendingPaths.size > 0) {
			await this.flush(pendingPaths);
		}
	}

	async flush(paths: AbsoluteFilePathSet, initial: boolean = false) {
		await this.flushLock.wrap(async () => {
			// We could be evicting a project as the result of a modification made inside of the watch callback
			// Ensure it's complete before we decide to flush
			await this.server.memoryFs.processingLock.wait();

			if (paths.size === 0 && !initial) {
				return;
			}

			await this.callback({paths, initial});
		});
	}

	setupEvents(): Array<EventSubscription> {
		const {memoryFs, server} = this;
		const subscriptions: Array<EventSubscription> = [];

		// Emitted when a file appears for the first time
		subscriptions.push(
			memoryFs.newFileEvent.subscribe((paths) => {
				this.flushPaths(paths);
			}),
		);

		subscriptions.push(
			server.refreshFileEvent.subscribe((paths) => {
				this.flushPaths(paths);
			}),
		);

		return subscriptions;
	}

	async init(): Promise<EventSubscription> {
		const {memoryFs} = this;
		const subs = this.setupEvents();
		const finalSubs = mergeEventSubscriptions([
			...subs,
			{
				unsubscribe: async () => {
					await this.flushLock.wait();
				},
			},
		]);

		try {
			const promises: Array<Promise<unknown>> = [];
			const batchPaths = new AbsoluteFilePathSet();
			this.batchPaths = batchPaths;

			// Determine what arguments are not available in the memory file system
			for (const arg of this.args) {
				// exists returns undefined when it's not available
				if (memoryFs.exists(arg) === undefined) {
					promises.push(this.server.projectManager.findProject(arg));
				} else {
					promises.push(this.flushPaths([arg]));
				}
			}

			await Promise.all(promises);
			this.batchPaths = undefined;
			await this.flush(batchPaths, true);
			await this.globber.get(true);

			return finalSubs;
		} catch (err) {
			await finalSubs.unsubscribe();
			throw err;
		}
	}
}
