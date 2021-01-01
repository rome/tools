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
} from "@internal/project";
import {
	Diagnostics,
	DiagnosticsProcessor,
	catchDiagnostics,
	descriptions,
} from "@internal/diagnostics";
import {EventQueue} from "@internal/events";
import {json} from "@internal/codec-config";
import {WorkerPartialManifest} from "../../common/bridges/WorkerBridge";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
} from "@internal/path";
import {
	FSStats,
	FSWatcher,
	exists,
	lstat,
	readDirectory,
	readFileText,
	watch,
} from "@internal/fs";
import {getFileHandlerFromPath} from "@internal/core";
import crypto = require("crypto");
import {FileNotFound} from "@internal/fs/FileNotFound";
import {markup} from "@internal/markup";
import {ReporterNamespace} from "@internal/cli-reporter";
import {GlobOptions, Globber} from "./glob";
import {VoidCallback} from "@internal/typescript-helpers";
import {GlobalLock} from "@internal/async";

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
	content: undefined | string;
};

type CrawlOptions = {
	reason: "watch" | "initial";
	watcherId?: number;
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

		this.logger = server.logger.namespace(markup`[MemoryFileSystem]`);

		this.watcherCounter = 0;
		this.watchPromises = new AbsoluteFilePathMap();
		this.watchers = new AbsoluteFilePathMap();
		this.activeWatcherIds = new Set();

		this.changedFileEvent = new EventQueue();
		this.deletedFileEvent = new EventQueue();
		this.newFileEvent = new EventQueue();

		this.processingLock = new GlobalLock();
		this.processingLock.attachLock(this.changedFileEvent.lock);
		this.processingLock.attachLock(this.deletedFileEvent.lock);
	}

	public changedFileEvent: EventQueue<ChangedFileEventItem>;
	public deletedFileEvent: EventQueue<AbsoluteFilePath>;
	public newFileEvent: EventQueue<AbsoluteFilePath>;
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
		path: AbsoluteFilePath;
		close: WatcherClose;
	}>;
	private activeWatcherIds: Set<number>;
	private watchPromises: AbsoluteFilePathMap<Promise<WatcherClose>>;

	// Used to maintain fake mtimes for file buffers
	private buffers: AbsoluteFilePathMap<SimpleStats>;

	isActiveWatcherId(id: undefined | number): boolean {
		return id === undefined || this.activeWatcherIds.has(id);
	}

	public async init() {
		await this.injectVirtualModules();
	}

	// Inject virtual modules so they are discoverable
	private async injectVirtualModules() {
		const files = this.server.virtualModules.getStatMap();

		for (const [path, {stats, content}] of files) {
			if (stats.isDirectory()) {
				this.directories.set(path, toSimpleStats(stats));
			} else {
				this.files.set(path, toSimpleStats(stats));
				this.addFileToDirectoryListing(path);

				if (isValidManifest(path)) {
					await this.declareManifest({
						content,
						diagnostics: this.server.createDisconnectedDiagnosticsProcessor([]),
						dirname: path.getParent(),
						path,
					});
				}
			}
		}
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

				const watcher = watch(
					directoryPath,
					{recursive, persistent: false},
					(eventType, filename) => {
						this.logger.info(
							markup`Raw fs.watch event in <emphasis>${directoryPath}</emphasis> type ${eventType} for ${String(
								filename,
							)}`,
						);

						if (filename === null) {
							// TODO not sure how we want to handle this?
							return;
						}

						const path = directoryPath.resolve(filename);
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
			const start = Date.now();
			const stats = await this.hardStat(projectDirectory);
			await this.addDirectory(
				projectDirectory,
				stats,
				{
					diagnostics,
					onFoundDirectory,
					reason: "initial",
				},
			);
			const took = Date.now() - start;
			logger.info(
				markup`[MemoryFileSystem] Finished initial crawl for <emphasis>${projectDirectory}</emphasis>. Added <number>${String(
					this.countFiles(projectDirectory),
				)}</number> files. Took <duration>${String(took)}</duration>`,
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

	public unwatchAll() {
		for (const {close} of this.watchers.values()) {
			close();
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

	public getPartialManifest(def: ManifestDefinition): WorkerPartialManifest {
		return {
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
			await Promise.all([
				this.deletedFileEvent.push(path),
				this.server.refreshFileEvent.push(path),
			]);
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
		if (parentListing !== undefined) {
			parentListing.delete(path);
		}
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

	public async watch(projectDirectory: AbsoluteFilePath): Promise<void> {
		const directoryLink = markup`<emphasis>${projectDirectory}</emphasis>`;

		// Defer if we're already currently initializing this project
		const cached = this.watchPromises.get(projectDirectory);
		if (cached !== undefined) {
			await cached;
			return undefined;
		}

		// Check if we're already watching this directory
		if (this.watchers.has(projectDirectory)) {
			return undefined;
		}

		// Check if we're already watching a parent directory
		for (const {path} of this.watchers.values()) {
			if (projectDirectory.isRelativeTo(path)) {
				this.logger.info(
					markup`Skipped crawl for ${directoryLink} because we're already watching the parent directory ${path}`,
				);
				return undefined;
			}
		}

		// Wait for other initializations
		await this.waitIfInitializingWatch(projectDirectory);

		// New watch target
		this.logger.info(markup`Adding new project directory ${directoryLink}`);

		// Remove watchers that are descedents of this directory as this watcher will handle them
		for (const [loc, {close, path}] of this.watchers) {
			if (path.isRelativeTo(projectDirectory)) {
				this.watchers.delete(loc);
				close();
			}
		}

		const diagnostics = this.server.createDiagnosticsProcessor({
			origins: [
				{
					category: "memory-fs",
					message: "Crawling project directory",
				},
			],
		});

		this.logger.info(markup`Watching ${directoryLink}`);
		const id = this.watcherCounter++;
		const promise = this.createWatcher(diagnostics, projectDirectory, id);
		this.watchPromises.set(projectDirectory, promise);

		await this.processingLock.wrap(async () => {
			const watcherClose = await promise;
			this.watchers.set(
				projectDirectory,
				{
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
		const stats = await lstat(path);
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

	public getMtimeNs(path: AbsoluteFilePath): bigint {
		const mtimeNs = this.maybeGetMtimeNs(path);
		if (mtimeNs === undefined) {
			throw new FileNotFound(path, "Not found in memory file system");
		} else {
			return mtimeNs;
		}
	}

	public getFileStats(path: AbsoluteFilePath): undefined | SimpleStats {
		return this.files.get(path);
	}

	public getFileStatsAssert(path: AbsoluteFilePath): SimpleStats {
		const stats = this.getFileStats(path);
		if (stats === undefined) {
			throw new FileNotFound(path, "Not found in memory file system");
		}
		return stats;
	}

	private isIgnored(path: AbsoluteFilePath, type: "directory" | "file"): boolean {
		const project = this.server.projectManager.findLoadedProject(path);
		if (project === undefined) {
			return false;
		}

		// If we're a file and don't have an extension handler so there's no reason for us to care about it
		if (
			type === "file" &&
			getFileHandlerFromPath(path, project.config) === undefined
		) {
			return true;
		}

		// Ensure we aren't in any of the default denylists
		const basename = path.getBasename();
		if (DEFAULT_DENYLIST.includes(basename)) {
			return true;
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
			content,
		}: DeclareManifestOpts,
	): Promise<void> {
		// Fetch the manifest
		const manifestRaw = content ?? (await readFileText(path));
		const hash = crypto.createHash("sha256").update(manifestRaw).digest("hex");

		const consumer = json.consumeValue({
			path,
			input: manifestRaw,
			consumeDiagnosticCategoryValue: "manifest",
		});

		const projects = await this.server.projectManager.getProjectHierarchyFromPath(
			path,
		);
		const {consumer: normalizeConsumer, diagnostics: rawDiagnostics} = consumer.capture();
		const manifest = await normalizeManifest(path, normalizeConsumer, projects);

		// If manifest is undefined then we failed to validate and have diagnostics
		if (rawDiagnostics.length > 0) {
			const normalizedDiagnostics: Diagnostics = rawDiagnostics.map((diag) => ({
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
				projectDirectory: directory,
				configPath: path,
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
				manifests: [{id: def.id, manifest: this.getPartialManifest(def)}],
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

	private async addDirectory(
		directoryPath: AbsoluteFilePath,
		stats: SimpleStats,
		opts: CrawlOptions,
	): Promise<boolean> {
		if (!this.hasStatsChanged(directoryPath, stats)) {
			return false;
		}

		// Check if this directory has been ignored
		if (this.isIgnored(directoryPath, "directory")) {
			return false;
		}

		if (opts.tick !== undefined) {
			opts.tick(directoryPath);
		}

		this.addFileToDirectoryListing(directoryPath);
		this.directories.set(directoryPath, stats);

		if (opts.onFoundDirectory !== undefined) {
			opts.onFoundDirectory(directoryPath);
		}

		// Crawl the directory
		const paths = await readDirectory(directoryPath);

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
		await Promise.all(Array.from(paths, declareItem));

		// If this directory is a project then mark it as initialized as we've crawled all their descendents
		const project = this.server.projectManager.getProjectFromPath(directoryPath);
		if (project !== undefined) {
			project.initialized = true;
		}

		return true;
	}

	public exists(path: AbsoluteFilePath): undefined | boolean {
		// if we have this in our cache then the file exists
		if (this.files.has(path) || this.directories.has(path)) {
			return true;
		}

		// If we're still performing an initial crawl of any path higher in the tree then we don't know if it exists yet
		for (const projectDirectory of this.watchPromises.keys()) {
			if (path.isRelativeTo(projectDirectory)) {
				return undefined;
			}
		}

		// if we're watching the parent directory then we'd have it in our cache if it existed
		const parent = path.getParent();
		if (this.directories.has(parent)) {
			return false;
		}

		return undefined;
	}

	public async existsHard(path: AbsoluteFilePath): Promise<boolean> {
		const resolvedExistence: undefined | boolean = this.exists(path);
		if (resolvedExistence === undefined) {
			return exists(path);
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

		const diagnostics = this.server.createDisconnectedDiagnosticsProcessor([
			{
				category: "memory-fs",
				message: originMessage,
			},
		]);

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
		if (!this.hasStatsChanged(path, stats)) {
			return false;
		}

		// Check if this file has been ignored
		if (this.isIgnored(path, "file")) {
			return false;
		}

		if (opts.tick !== undefined) {
			opts.tick(path);
		}

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

			await Promise.all([
				this.server.refreshFileEvent.push(path),
				this.changedFileEvent.push({path, oldStats, newStats: stats}),
			]);

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
						filename: path.join(),
					},
				});
			}

			await projectManager.addDiskProject({
				// Get the directory above .config
				projectDirectory: dirname.getParent(),
				configPath: path,
			});
		}

		if (isValidManifest(path)) {
			await this.declareManifest({
				content: undefined,
				diagnostics: opts.diagnostics,
				dirname,
				path,
			});
		}

		if (isNew) {
			await this.newFileEvent.push(path);
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
