/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {
	ManifestDefinition,
	manifestNameToString,
	normalizeManifest,
} from "@internal/codec-js-manifest";
import {
	PROJECT_CONFIG_DIRECTORY,
	PROJECT_CONFIG_FILENAMES,
	PROJECT_CONFIG_PACKAGE_JSON_FIELD,
	ProjectDefinition,
} from "@internal/project";
import {
	Diagnostic,
	DiagnosticsProcessor,
	catchDiagnostics,
	descriptions,
} from "@internal/diagnostics";
import {json} from "@internal/codec-config";
import {WorkerPartialManifest} from "@internal/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createRelativePath,
} from "@internal/path";
import {CachedFileReader, FSStats, FSWatcher, FileNotFound} from "@internal/fs";
import {markup} from "@internal/markup";
import {ReporterNamespace} from "@internal/cli-reporter";
import {GlobOptions, Globber} from "./glob";
import {VoidCallback} from "@internal/typescript-helpers";
import {GlobalLock, promiseAllFrom} from "@internal/async";
import {DurationMeasurer} from "@internal/numbers";
import crypto = require("crypto");
import {createResourceFromCallback} from "@internal/resources";

// Paths that we will under no circumstance want to include
const DEFAULT_DENYLIST = [
	".hg",
	".git",
	".idea",
	"node_modules/.staging",
	"node_modules/.cache",
];

function isValidManifest(path: AbsoluteFilePath): boolean {
	if (path.getBasename() !== "package.json") {
		return false;
	}

	// If a manifest is in node_modules, then make sure we're directly inside
	// a directory in node_modules.
	//
	// For unscoped package, the segments should be:
	//   -1: package.json
	//   -2: module directory
	//   -3: node_modules
	//
	// For scoped package (@scope/some-module), the segments should be:
	//   -1: package.json
	//   -2: module directory
	//   -3: scope directory
	//   -4: node_modules
	const segments = path.getSegments();
	if (segments.includes("node_modules")) {
		// Unscoped package
		if (segments[segments.length - 3] === "node_modules") {
			return true;
		}

		// Scoped module
		if (
			segments[segments.length - 4] === "node_modules" &&
			segments[segments.length - 3].startsWith("@")
		) {
			return true;
		}

		return false;
	}

	return true;
}

// Whenever we're performing an operation on a set of files, always do these first as they may influence how the rest are processed
const PRIORITY_FILES = new Set([PROJECT_CONFIG_DIRECTORY, "package.json"]);

type DeclareManifestOpts = {
	diagnostics: DiagnosticsProcessor;
	dirname: AbsoluteFilePath;
	path: AbsoluteFilePath;
	isPartialProject: boolean;
	reader: CachedFileReader;
};

type CrawlOptions = {
	reason: "watch" | "initial";
	reader: CachedFileReader;
	watcherId?: number;
	partialAllowlist?: AbsoluteFilePath[];
	diagnostics: DiagnosticsProcessor;
	onFoundDirectory?: (path: AbsoluteFilePath) => void;
	tick?: (path: AbsoluteFilePath) => void;
};

export type StatsType = "unknown" | "directory" | "file";

export type SimpleStats = {
	size: bigint;
	mtimeNs: bigint;
	type: StatsType;
};

export type WatcherClose = VoidCallback;

export type ChangedFileEventItem = {
	path: AbsoluteFilePath;
	oldStats: undefined | SimpleStats;
	newStats: SimpleStats;
};

function toSimpleStats(stats: FSStats): SimpleStats {
	let type: StatsType = "unknown";
	if (stats.isDirectory()) {
		type = "directory";
	} else if (stats.isFile()) {
		type = "file";
	}

	return {
		type,
		size: stats.size,
		mtimeNs: stats.mtimeNs,
	};
}

export default class MemoryFileSystem {
	constructor(server: Server) {
		this.server = server;

		this.directoryListings = new AbsoluteFilePathMap();
		this.directories = new AbsoluteFilePathMap();
		this.files = new AbsoluteFilePathMap();
		this.buffers = new AbsoluteFilePathMap();

		this.manifestCounter = 0;
		this.manifests = new AbsoluteFilePathMap();

		this.logger = server.logger.namespace(markup`MemoryFileSystem`);

		this.watcherCounter = 0;
		this.watchPromises = new AbsoluteFilePathMap();
		this.watchers = new AbsoluteFilePathMap();
		this.activeWatcherIds = new Set();

		this.processingLock = new GlobalLock();

		server.resources.add(
			createResourceFromCallback(
				"MemoryFileSystem",
				() => {
					return this.end();
				},
			),
		);
	}

	public processingLock: GlobalLock;
	private manifestCounter: number;
	private watcherCounter: number;
	private server: Server;
	private directoryListings: AbsoluteFilePathMap<AbsoluteFilePathMap<AbsoluteFilePath>>;
	private directories: AbsoluteFilePathMap<SimpleStats>;
	private files: AbsoluteFilePathMap<SimpleStats>;
	private manifests: AbsoluteFilePathMap<ManifestDefinition>;
	private logger: ReporterNamespace;

	private watchers: AbsoluteFilePathMap<{
		partial: boolean;
		path: AbsoluteFilePath;
		close: WatcherClose;
	}>;
	private activeWatcherIds: Set<number>;
	private watchPromises: AbsoluteFilePathMap<Promise<WatcherClose>>;

	// Used to maintain fake mtimes for file buffers
	private buffers: AbsoluteFilePathMap<SimpleStats>;

	private isActiveWatcherId(id: undefined | number): boolean {
		return id === undefined || this.activeWatcherIds.has(id);
	}

	public async end() {
		await promiseAllFrom(
			this.watchers.values(),
			(watcher) => {
				return watcher.close();
			},
		);

		this.watchers.clear();
		this.files.clear();
		this.directories.clear();
		this.manifests.clear();
		this.buffers.clear();
		this.directoryListings.clear();
	}

	// Given a path that exists in our files map, return the instance we have stored that matches the input path
	// This allows us to use the memoized and cached path derivatives
	public coalescePath(path: AbsoluteFilePath): AbsoluteFilePath {
		return this.files.normalizeKey(path);
	}

	public async init() {
		await this.injectVirtualModules();
	}

	// Inject virtual modules so they are discoverable
	private async injectVirtualModules() {
		const files = this.server.virtualModules.getStatMap();
		const reader = new CachedFileReader();

		for (const [path, entry] of files) {
			if (entry.type === "directory") {
				this.directories.set(path, toSimpleStats(entry.stats));
			} else {
				reader.cache(path, entry.content);
				this.files.set(path, toSimpleStats(entry.stats));
				this.addFileToDirectoryListing(path);

				if (isValidManifest(path)) {
					await this.declareManifest({
						isPartialProject: false,
						diagnostics: this.server.createDisconnectedDiagnosticsProcessor(),
						dirname: path.getParent(),
						path,
						reader,
					});
				}
			}
		}
	}

	public hasAnyBuffers(): boolean {
		return this.buffers.size > 0;
	}

	public hasBuffer(path: AbsoluteFilePath): boolean {
		return this.buffers.has(path);
	}

	public addBuffer(path: AbsoluteFilePath, content: string): bigint {
		const mtime = BigInt(Date.now()) * 1000000n;

		this.buffers.set(
			path,
			{
				type: "file",
				size: BigInt(content.length),
				mtimeNs: mtime,
			},
		);

		return mtime;
	}

	public clearBuffer(path: AbsoluteFilePath) {
		this.buffers.delete(path);
	}

	private async createWatcher(
		diagnostics: DiagnosticsProcessor,
		projectDirectory: AbsoluteFilePath,
		id: number,
		partialAllowlist?: AbsoluteFilePath[],
	): Promise<WatcherClose> {
		const {server} = this;
		const {logger} = server;
		const projectDirectoryMarkup = markup`<emphasis>${projectDirectory}</emphasis>`;

		// Create activity spinners for all connected reporters
		const activity = this.server.connectedReporters.progress({
			initDelay: 1_000,
			title: markup`Adding project ${projectDirectoryMarkup}`,
		});

		const watchers: AbsoluteFilePathMap<FSWatcher> = new AbsoluteFilePathMap();
		this.activeWatcherIds.add(id);

		try {
			const onFoundDirectory = (directoryPath: AbsoluteFilePath) => {
				if (watchers.has(directoryPath)) {
					return;
				}

				let recursive = true;

				if (process.platform === "linux" || process.platform === "android") {
					// Node on Linux doesn't support recursive directory watching so we need an fs.watch for every directory...
					recursive = false;
				} else if (!directoryPath.equal(projectDirectory)) {
					// If we're on any other platform then only watch the root project directory
					return;
				}

				const watcher = directoryPath.watch(
					{recursive, persistent: false},
					(eventType, filename) => {
						this.logger.info(
							markup`Raw fs.watch event in <emphasis>${directoryPath}</emphasis> type ${eventType} for <emphasis>${String(
								filename,
							)}</emphasis>`,
						);

						if (filename === null) {
							// TODO not sure how we want to handle this?
							return;
						}

						const path = directoryPath.resolve(createRelativePath(filename));
						this.refreshPath(
							path,
							{onFoundDirectory, watcherId: id},
							"Processing fs.watch changes",
						);
					},
				);
				watchers.set(directoryPath, watcher);
			};

			// No need to call watch() on the projectDirectory since it will call us

			// Perform an initial crawl
			const start = new DurationMeasurer();
			const stats = await this.hardStat(projectDirectory);
			await this.addDirectory(
				projectDirectory,
				stats,
				{
					partialAllowlist,
					diagnostics,
					onFoundDirectory,
					reason: "initial",
					reader: new CachedFileReader(),
				},
			);
			logger.info(
				markup`[MemoryFileSystem] Finished initial crawl for <emphasis>${projectDirectory}</emphasis>. Added <number>${String(
					this.countFiles(projectDirectory),
				)}</number> files. Took ${start.since()}`,
			);
		} finally {
			activity.end();
		}

		return () => {
			this.activeWatcherIds.delete(id);
			for (const watcher of watchers.values()) {
				watcher.close();
			}
		};
	}

	public close(dirPath: AbsoluteFilePath) {
		const watcher = this.watchers.get(dirPath);
		if (watcher === undefined) {
			return;
		}

		this.watchers.delete(dirPath);
		watcher.close();
	}

	public unwatch(dirPath: AbsoluteFilePath) {
		this.close(dirPath);

		// Go through and clear all files and directories from our internal maps
		// NOTE: We deliberately do not call 'deletedFileEvent' as the code that
		// calls us will already be cleaning up
		let queue: AbsoluteFilePath[] = [dirPath];
		while (queue.length > 0) {
			const path = queue.pop()!;

			this.directories.delete(path);
			this.manifests.delete(path);
			this.files.delete(path);

			const listing = this.directoryListings.get(path);
			if (listing !== undefined) {
				this.directoryListings.delete(path);
				queue = queue.concat(Array.from(listing.values()));
			}
		}
	}

	public readdir(path: AbsoluteFilePath): Iterable<AbsoluteFilePath> {
		const listing = this.directoryListings.get(path);
		if (listing === undefined) {
			return [];
		} else {
			return listing.values();
		}
	}

	public isDirectory(path: AbsoluteFilePath): boolean {
		return this.directories.has(path);
	}

	public isFile(path: AbsoluteFilePath): boolean {
		return this.files.has(path);
	}

	public getManifestDefinition(
		dirname: AbsoluteFilePath,
	): undefined | ManifestDefinition {
		return this.manifests.get(dirname);
	}

	public getOwnedManifest(
		path: AbsoluteFilePath,
	): undefined | ManifestDefinition {
		for (const dir of path.getChain()) {
			const def = this.server.memoryFs.getManifestDefinition(dir);
			if (def !== undefined) {
				return def;
			}
		}
		return undefined;
	}

	public getPartialManifest(
		def: ManifestDefinition,
		project: ProjectDefinition,
	): WorkerPartialManifest {
		return {
			project: project.id,
			path: def.path,
			hash: def.hash,
			type: def.manifest.type,
		};
	}

	private addFileToDirectoryListing(path: AbsoluteFilePath): void {
		const dirname = path.getParent();
		let listing = this.directoryListings.get(dirname);
		if (listing === undefined) {
			listing = new AbsoluteFilePathMap();
			this.directoryListings.set(dirname, listing);
		}
		listing.set(path, path);
	}

	private async handleDeletion(path: AbsoluteFilePath): Promise<void> {
		// If a directory then evict all children
		const directoryInfo = this.directories.get(path);
		if (directoryInfo !== undefined) {
			this.directories.delete(path);

			const listing = this.directoryListings.get(path);
			if (listing !== undefined) {
				this.directoryListings.delete(path);
				for (const path of listing.values()) {
					await this.handleDeletion(path);
				}
			}
		}

		this.logger.info(markup`File deleted: ${path}`);

		// Wait for any subscribers that might need the file's stats
		// Only emit these events for files
		if (directoryInfo === undefined) {
			await this.server.refreshFileEvent.push({
				type: "DELETED",
				path,
			});
		}

		// Remove from 'all possible caches
		this.files.delete(path);

		// If this is a manifest filename then clear it from 'any possible package and our internal module map
		const basename = path.getBasename();
		if (basename === "package.json") {
			this.handleDeletedManifest(path);
		}

		// Remove from 'parent directory listing
		const dirname = path.getParent();
		const parentListing = this.directoryListings.get(dirname);

		parentListing?.delete(path);
	}

	private handleDeletedManifest(path: AbsoluteFilePath): void {
		const directory = path.getParent();
		const def = this.manifests.get(directory);
		if (def !== undefined) {
			this.manifests.delete(directory);
		}
	}

	public async waitIfInitializingWatch(
		projectDirectoryPath: AbsoluteFilePath,
	): Promise<void> {
		// Defer if we're initializing a parent directory
		for (const [path, promise] of this.watchPromises) {
			if (projectDirectoryPath.isRelativeTo(path)) {
				await promise;
				return;
			}
		}

		// Wait if we're initializing descendents
		for (const [path, promise] of this.watchPromises) {
			if (path.isRelativeTo(projectDirectoryPath)) {
				await promise;
			}
		}
	}

	public async watch(
		projectDirectory: AbsoluteFilePath,
		target: AbsoluteFilePath = projectDirectory,
		partial: boolean = false,
	): Promise<void> {
		const directoryLink = markup`<emphasis>${projectDirectory}</emphasis>`;

		// Defer if we're already currently initializing this project
		const cached = this.watchPromises.get(projectDirectory);
		if (cached !== undefined) {
			await cached;
		}

		// Check if we're already watching this directory
		const existingWatcher = this.watchers.get(projectDirectory);
		if (existingWatcher !== undefined) {
			if (existingWatcher.partial) {
				// Already loaded but only partially so refresh it
				existingWatcher.close();
			} else {
				// Already loaded
				return undefined;
			}
		}

		// Check if we're already watching a parent directory
		for (const {path, close, partial} of this.watchers.values()) {
			if (projectDirectory.isRelativeTo(path)) {
				if (partial) {
					// If this directory was loaded as partial then reload it as complete
					close();
					return this.watch(path);
				} else {
					this.logger.info(
						markup`Skipped crawl for ${directoryLink} because we're already watching the parent directory ${path}`,
					);
					return undefined;
				}
			}
		}

		// Wait for other initializations
		await this.waitIfInitializingWatch(projectDirectory);

		// New watch target
		if (partial) {
			this.logger.info(
				markup`Adding new partial project directory ${directoryLink} with a target of ${target}`,
			);
		} else {
			this.logger.info(markup`Adding new project directory ${directoryLink}`);
		}

		// Remove watchers that are descedents of this directory as this watcher will handle them
		for (const [loc, {close, path}] of this.watchers) {
			if (path.isRelativeTo(projectDirectory)) {
				this.watchers.delete(loc);
				close();

				// Don't allow partial watching as we need at a minimum to include this directory
				partial = false;
			}
		}

		const diagnostics = this.server.createDiagnosticsProcessor({
			origin: {
				entity: "MemoryFileSystem",
				message: "Crawling project directory",
			},
		});

		this.logger.info(markup`Watching ${directoryLink}`);
		const id = this.watcherCounter++;
		const promise = this.createWatcher(
			diagnostics,
			projectDirectory,
			id,
			partial ? [target] : undefined,
		);
		this.watchPromises.set(projectDirectory, promise);

		await this.processingLock.wrap(async () => {
			const watcherClose = await promise;
			this.watchers.set(
				projectDirectory,
				{
					partial,
					path: projectDirectory,
					close: watcherClose,
				},
			);
			this.watchPromises.delete(projectDirectory);
		});

		diagnostics.maybeThrowDiagnosticsError();
	}

	// Query actual file system for stats
	private async hardStat(path: AbsoluteFilePath): Promise<SimpleStats> {
		const stats = await path.lstat();
		return toSimpleStats(stats);
	}

	public maybeGetMtimeNs(path: AbsoluteFilePath): undefined | bigint {
		const stats = this.buffers.get(path) || this.files.get(path);
		if (stats === undefined) {
			return undefined;
		} else {
			return stats.mtimeNs;
		}
	}

	public getFileStats(path: AbsoluteFilePath): undefined | SimpleStats {
		return this.buffers.get(path) ?? this.files.get(path);
	}

	public getFileStatsAssert(path: AbsoluteFilePath): SimpleStats {
		const stats = this.getFileStats(path);
		if (stats === undefined) {
			throw new FileNotFound(path, "Not found in memory file system");
		}
		return stats;
	}

	private isIgnored(path: AbsoluteFilePath, type: "directory" | "file"): boolean {
		type;

		// Ensure we aren't in any of the default denylists
		const basename = path.getBasename();
		if (DEFAULT_DENYLIST.includes(basename)) {
			return true;
		}

		const project = this.server.projectManager.findLoadedProject(path);
		if (project === undefined) {
			return false;
		}

		return false;
	}

	private isInsideProject(path: AbsoluteFilePath): boolean {
		return path.getSegments().includes("node_modules") === false;
	}

	// This is a wrapper around _declareManifest as it can produce diagnostics
	private async declareManifest(opts: DeclareManifestOpts): Promise<void> {
		const {diagnostics} = await catchDiagnostics(() => {
			return this.declareManifestWithPossibleDiagnosticsThrow(opts);
		});

		if (diagnostics !== undefined) {
			opts.diagnostics.addDiagnostics(diagnostics);
		}
	}

	private async declareManifestWithPossibleDiagnosticsThrow(
		{
			path,
			diagnostics,
			isPartialProject,
			reader,
		}: DeclareManifestOpts,
	): Promise<void> {
		// Fetch the manifest
		const manifestRaw = await reader.readFileText(path);
		const hash = crypto.createHash("sha256").update(manifestRaw).digest("hex");
		const consumer = json.consumeValue({
			path,
			input: manifestRaw,
			consumeDiagnosticCategoryValue: "manifest",
		});

		const projects = await this.server.projectManager.getProjectHierarchyFromPath(
			path,
		);
		let checkDependenciesAndLicense = false;
		const mainProject = await this.server.projectManager.findLoadedProject(path);
		if (mainProject) {
			checkDependenciesAndLicense = mainProject.config.dependencies.enabled;
		}
		const {consumer: normalizeConsumer, diagnostics: rawDiagnostics} = consumer.capture();
		const manifest = await normalizeManifest({
			path,
			consumer: normalizeConsumer,
			projects,
			checkDependenciesAndLicense,
		});

		// If manifest is undefined then we failed to validate and have diagnostics
		if (rawDiagnostics.length > 0) {
			const normalizedDiagnostics: Diagnostic[] = rawDiagnostics.map((diag) => ({
				...diag,
				description: {
					...diag.description,
					advice: [
						...diag.description.advice,
						{
							type: "log",
							category: "info",
							text: markup`Error occurred for package <emphasis>${manifestNameToString(
								manifest.name,
							)}</emphasis> at <emphasis>${path.getParent()}</emphasis>`,
						},
					],
				},
			}));
			diagnostics.addDiagnostics(normalizedDiagnostics);
			return;
		}

		const directory = path.getParent();
		const manifestId = this.manifestCounter++;
		const def: ManifestDefinition = {
			id: manifestId,
			path,
			directory,
			consumer,
			manifest,
			hash,
		};

		this.manifests.set(directory, def);

		// If we aren't in node_modules then this is a project package
		const isProjectPackage = this.isInsideProject(path);
		const {projectManager} = this.server;

		if (isProjectPackage && consumer.has(PROJECT_CONFIG_PACKAGE_JSON_FIELD)) {
			await projectManager.addDiskProject({
				reader,
				projectDirectory: directory,
				configPath: path,
				isPartial: isPartialProject,
			});
		}

		const project = projectManager.findLoadedProject(path);
		if (project === undefined) {
			// Project failed to load. We'll display the errors but failing hard here with assertProjectExisting will hide them.
			return;
		}

		projectManager.declareManifest(project, isProjectPackage, def, diagnostics);

		// Tell all workers of our discovery
		for (const worker of this.server.workerManager.getWorkers()) {
			worker.bridge.events.updateManifests.send({
				manifests: new Map([[def.id, this.getPartialManifest(def, project)]]),
			});
		}
	}

	private countFiles(directory: AbsoluteFilePath): number {
		let count: number = 0;

		const listing = this.directoryListings.get(directory);
		if (listing !== undefined) {
			for (const file of listing.keys()) {
				count++;
				count += this.countFiles(file);
			}
		}

		return count;
	}

	private hasStatsChanged(
		path: AbsoluteFilePath,
		newStats: SimpleStats,
	): boolean {
		const oldStats = this.directories.get(path) || this.files.get(path);
		return oldStats === undefined || newStats.mtimeNs !== oldStats.mtimeNs;
	}

	private getPartialAllowlist(
		path: AbsoluteFilePath,
		opts: CrawlOptions,
	): undefined | (AbsoluteFilePath[]) {
		// No use doing these potentially expensive path checks if we don't even have an allowlist to return
		if (opts.partialAllowlist === undefined) {
			return undefined;
		}

		// Always process and include a package.json or other priority file
		if (PRIORITY_FILES.has(path.getBasename())) {
			return undefined;
		}

		// Also always include files inside of those blessed directory names
		if (path.hasParent() && PRIORITY_FILES.has(path.getParent().getBasename())) {
			return undefined;
		}

		return opts.partialAllowlist;
	}

	private async addDirectory(
		directoryPath: AbsoluteFilePath,
		stats: SimpleStats,
		opts: CrawlOptions,
	): Promise<boolean> {
		const partialAllowlist = this.getPartialAllowlist(directoryPath, opts);
		if (partialAllowlist !== undefined) {
			let allowed = false;
			for (const onlyAllowPath of partialAllowlist) {
				if (onlyAllowPath.isRelativeTo(directoryPath)) {
					allowed = true;
					break;
				}
			}
			if (!allowed) {
				return false;
			}
		}

		if (!this.hasStatsChanged(directoryPath, stats)) {
			return false;
		}

		// Check if this directory has been ignored
		if (this.isIgnored(directoryPath, "directory")) {
			return false;
		}

		opts.tick?.(directoryPath);

		this.addFileToDirectoryListing(directoryPath);
		this.directories.set(directoryPath, stats);

		opts.onFoundDirectory?.(directoryPath);

		// Crawl the directory
		const paths = await directoryPath.readDirectory();

		// Declare the file
		const declareItem = async (path: AbsoluteFilePath) => {
			// Watcher could have been closed by an event
			if (!this.isActiveWatcherId(opts.watcherId)) {
				return;
			}

			const stats = await this.hardStat(path);
			if (stats.type === "file") {
				await this.addFile(path, stats, opts);
			} else if (stats.type === "directory") {
				await this.addDirectory(path, stats, opts);
			}
		};

		// Give priority to package.json in case we want to derive something from the project config
		for (const priorityBasename of PRIORITY_FILES) {
			for (const file of paths) {
				if (priorityBasename === file.getBasename()) {
					paths.delete(file);
					await declareItem(file);
				}
			}
		}

		// Add the rest of the items
		await promiseAllFrom(paths, declareItem);

		// If this directory is a project then mark it as initialized as we've crawled all their descendents
		const project = this.server.projectManager.getProjectFromPath(directoryPath);
		if (project !== undefined) {
			project.initialized = true;
		}

		return true;
	}

	public exists(path: AbsoluteFilePath): undefined | boolean {
		// if we have this in our cache then the file exists
		if (
			this.buffers.has(path) ||
			this.files.has(path) ||
			this.directories.has(path)
		) {
			return true;
		}

		// If we're still performing an initial crawl of any path higher in the tree then we don't know if it exists yet
		for (const projectDirectory of this.watchPromises.keys()) {
			if (path.isRelativeTo(projectDirectory)) {
				return undefined;
			}
		}

		// If we're watching the parent directory then we'd have it in our cache if it existed
		if (path.hasParent()) {
			const parent = path.getParent();
			if (this.directories.has(parent)) {
				return false;
			}
		}

		return undefined;
	}

	public async existsHard(path: AbsoluteFilePath): Promise<boolean> {
		const resolvedExistence: undefined | boolean = this.exists(path);
		if (resolvedExistence === undefined) {
			return path.exists();
		} else {
			return resolvedExistence;
		}
	}

	public async refreshPath(
		path: AbsoluteFilePath,
		customCrawlOpts: Partial<CrawlOptions> = {},
		originMessage: string = "maybeRefreshFile",
	) {
		if (!this.isActiveWatcherId(customCrawlOpts.watcherId)) {
			return;
		}

		const diagnostics = this.server.createDisconnectedDiagnosticsProcessor({
			entity: "MemoryFileSystem",
			message: originMessage,
		});

		let newStats;
		try {
			newStats = await this.hardStat(path);
		} catch (err) {
			if (err.code === "ENOENT") {
				// Only call handleDeletion if we think this file still exists
				if (this.exists(path)) {
					await this.handleDeletion(path);
				}
			} else {
				throw err;
			}
		}
		if (newStats === undefined) {
			// Deleted
			return;
		}

		const crawlOpts: CrawlOptions = {
			reason: "watch",
			diagnostics,
			reader: new CachedFileReader(),
			...customCrawlOpts,
		};

		if (newStats.type === "directory") {
			await this.addDirectory(path, newStats, crawlOpts);
		} else if (newStats.type === "file") {
			await this.addFile(path, newStats, crawlOpts);
		}
	}

	private async addFile(
		path: AbsoluteFilePath,
		stats: SimpleStats,
		opts: CrawlOptions,
	): Promise<boolean> {
		const partialAllowlist = this.getPartialAllowlist(path, opts);
		if (partialAllowlist !== undefined) {
			let allowed = false;
			for (const onlyAllowPath of partialAllowlist) {
				if (path.equal(onlyAllowPath)) {
					allowed = true;
					break;
				}
			}
			if (!allowed) {
				return false;
			}
		}

		if (!this.hasStatsChanged(path, stats)) {
			return false;
		}

		// Check if this file has been ignored
		if (this.isIgnored(path, "file")) {
			return false;
		}

		opts.tick?.(path);

		const isNew = !this.files.has(path);
		this.files.set(path, stats);
		this.addFileToDirectoryListing(path);

		const basename = path.getBasename();
		const dirname = path.getParent();

		// Warn about potentially incorrect Rome config filenames
		const {projectManager} = this.server;
		projectManager.checkPathForIncorrectConfig(path, opts.diagnostics);

		// Detect file changes
		const oldStats = this.getFileStats(path);
		if (oldStats !== undefined && opts.reason === "watch") {
			this.logger.info(markup`File change: <emphasis>${path}</emphasis>`);

			await this.server.refreshFileEvent.push({
				type: "DISK_UPDATE",
				path,
				oldStats,
				newStats: stats,
			});

			// Watcher could have been closed by an event
			if (!this.isActiveWatcherId(opts.watcherId)) {
				return false;
			}
		}

		//this.logger.info(markup`Found: <emphasis>${path}</emphasis>`);

		// Add project if this is a config
		if (
			dirname.getBasename() === PROJECT_CONFIG_DIRECTORY &&
			PROJECT_CONFIG_FILENAMES.includes(basename)
		) {
			if (projectManager.hasLoadedProjectDirectory(dirname.getParent())) {
				opts.diagnostics.addDiagnostic({
					description: descriptions.PROJECT_MANAGER.MULTIPLE_CONFIGS,
					location: {
						path,
					},
				});
			}

			await projectManager.addDiskProject({
				reader: opts.reader,
				isPartial: opts.partialAllowlist !== undefined,
				// Get the directory above .config
				projectDirectory: dirname.getParent(),
				configPath: path,
			});
		}

		if (isValidManifest(path)) {
			await this.declareManifest({
				reader: opts.reader,
				isPartialProject: opts.partialAllowlist !== undefined,
				diagnostics: opts.diagnostics,
				dirname,
				path,
			});
		}

		if (isNew) {
			await this.server.refreshFileEvent.push({type: "CREATED", path});
		}

		return true;
	}

	public glob(
		arg: AbsoluteFilePath,
		opts: Omit<GlobOptions, "args"> = {},
	): AbsoluteFilePathSet {
		const globber = new Globber(
			this.server,
			{
				...opts,
				args: [arg],
			},
		);
		return globber.search(arg);
	}
}
