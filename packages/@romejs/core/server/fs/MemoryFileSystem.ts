/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {
	Manifest,
	ManifestDefinition,
	normalizeManifest,
} from "@romejs/codec-js-manifest";
import {
	PathPatterns,
	matchPathPatterns,
	parsePathPattern,
} from "@romejs/path-match";
import {
	ProjectConfigCategoriesWithIgnore,
	ProjectDefinition,
	ROME_CONFIG_FILENAMES,
	ROME_CONFIG_PACKAGE_JSON_FIELD,
} from "@romejs/project";
import {DiagnosticsProcessor, catchDiagnostics} from "@romejs/diagnostics";
import {Event} from "@romejs/events";
import {consumeJSON} from "@romejs/codec-json";
import {humanizeNumber} from "@romejs/string-utils";
import {WorkerPartialManifest} from "../../common/bridges/WorkerBridge";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
} from "@romejs/path";
import {exists, lstat, readDirectory, readFileText, watch} from "@romejs/fs";
import {getFileHandler} from "../../common/file-handlers/index";
import {markup} from "@romejs/string-markup";
import crypto = require("crypto");
import fs = require("fs");
import {FileNotFound} from "@romejs/core/common/FileNotFound";

const DEFAULT_DENYLIST = [".hg", ".git"];

const GLOB_IGNORE: PathPatterns = [
	parsePathPattern({input: "node_modules"}),
	parsePathPattern({input: ".git"}),
	parsePathPattern({input: ".hg"}),
];

function concatGlobIgnore(patterns: PathPatterns): PathPatterns {
	// If there are any negate patterns then it'll never include GLOB_IGNORE
	for (const pattern of patterns) {
		if (pattern.type === "PathPattern" && pattern.negate) {
			return patterns;
		}
	}

	return [...GLOB_IGNORE, ...patterns];
}

function isValidManifest(path: AbsoluteFilePath): boolean {
	if (path.getBasename() !== "package.json") {
		return false;
	}

	// If a manifest is in node_modules, then make sure we're directly inside
	// a folder in node_modules.
	//
	// For unscoped package, the segments should be:
	//   -1: package.json
	//   -2: module folder
	//   -3: node_modules
	//
	// For scoped package (@scope/some-module), the segments should be:
	//   -1: package.json
	//   -2: module folder
	//   -3: scope folder
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
const PRIORITY_FILES = new Set([...ROME_CONFIG_FILENAMES, "package.json"]);

type DeclareManifestOpts = {
	diagnostics: DiagnosticsProcessor;
	dirname: AbsoluteFilePath;
	path: AbsoluteFilePath;
};

type CrawlOptions = {
	reason: "watch" | "initial";
	diagnostics: DiagnosticsProcessor;
	onFoundDirectory?: (path: AbsoluteFilePath) => void;
	tick?: (path: AbsoluteFilePath) => void;
};

export type StatsType = "unknown" | "directory" | "file";

export type Stats = {
	size: number;
	mtime: number;
	type: StatsType;
};

export type WatcherClose = () => void;

export type MemoryFSGlobOptions = {
	extensions?: Array<string>;
	overrideIgnore?: PathPatterns;
	configCategory?: ProjectConfigCategoriesWithIgnore;
	test?: (path: AbsoluteFilePath) => boolean;
};

async function createWatcher(
	memoryFs: MemoryFileSystem,
	diagnostics: DiagnosticsProcessor,
	projectFolder: AbsoluteFilePath,
): Promise<WatcherClose> {
	const {logger} = memoryFs.server;

	// Create activity spinners for all connected reporters
	const activity = memoryFs.server.connectedReporters.progress({
		initDelay: 1_000,
		title: `Adding project ${projectFolder.toMarkup()}`,
	});

	const watchers: AbsoluteFilePathMap<fs.FSWatcher> = new AbsoluteFilePathMap();

	try {
		function onFoundDirectory(folderPath: AbsoluteFilePath) {
			if (watchers.has(folderPath)) {
				return;
			}

			let recursive = true;

			if (process.platform === "linux") {
				// Node on Linux doesn't support recursive directory watching so we need an fs.watch for every directory...
				recursive = false;
			} else if (!folderPath.equal(projectFolder)) {
				// If we're on any other platform then only watch the root project folder
				return;
			}

			const watcher = watch(
				folderPath,
				{recursive, persistent: false},
				(eventType, filename) => {
					if (filename === null) {
						// TODO not sure how we want to handle this?
						return;
					}

					const path = folderPath.resolve(filename);

					memoryFs.hardStat(path).then((newStats) => {
						const diagnostics = memoryFs.server.createDisconnectedDiagnosticsProcessor([
							{
								category: "memory-fs",
								message: "Processing fs.watch changes",
							},
						]);

						if (newStats.type === "file") {
							memoryFs.addFile(
								path,
								newStats,
								{
									reason: "watch",
									diagnostics,
								},
							);
						} else if (newStats.type === "directory") {
							memoryFs.addDirectory(
								path,
								newStats,
								{
									reason: "watch",
									diagnostics,
									onFoundDirectory,
								},
							);
						}
					}).catch((err) => {
						if (err.code === "ENOENT") {
							memoryFs.handleDeletion(path);
						} else {
							throw err;
						}
					});
				},
			);
			watchers.set(folderPath, watcher);
		}

		// No need to call watch() on the projectFolder since it will call us

		// Perform an initial crawl
		const stats = await memoryFs.hardStat(projectFolder);
		await memoryFs.addDirectory(
			projectFolder,
			stats,
			{
				diagnostics,
				onFoundDirectory,
				reason: "initial",
			},
		);
		logger.info(
			`[MemoryFileSystem] Finished initial crawl for ${projectFolder.toMarkup()} - added ${humanizeNumber(
				memoryFs.countFiles(projectFolder),
			)} files`,
		);
	} finally {
		activity.end();
	}

	return () => {
		for (const watcher of watchers.values()) {
			watcher.close();
		}
	};
}

export default class MemoryFileSystem {
	constructor(server: Server) {
		this.server = server;

		this.watchPromises = new AbsoluteFilePathMap();
		this.directoryListings = new AbsoluteFilePathMap();
		this.directories = new AbsoluteFilePathMap();
		this.files = new AbsoluteFilePathMap();
		this.manifests = new AbsoluteFilePathMap();
		this.watchers = new AbsoluteFilePathMap();
		this.buffers = new AbsoluteFilePathMap();
		this.manifestCounter = 0;

		this.changedFileEvent = new Event({
			name: "MemoryFileSystem.changedFile",
			onError: server.onFatalErrorBound,
		});
		this.deletedFileEvent = new Event({
			name: "MemoryFileSystem.deletedFile",
			onError: server.onFatalErrorBound,
		});
	}

	manifestCounter: number;
	server: Server;
	directoryListings: AbsoluteFilePathMap<AbsoluteFilePathMap<AbsoluteFilePath>>;
	directories: AbsoluteFilePathMap<Stats>;
	files: AbsoluteFilePathMap<Stats>;
	manifests: AbsoluteFilePathMap<ManifestDefinition>;

	watchers: AbsoluteFilePathMap<{
		path: AbsoluteFilePath;
		close: WatcherClose;
	}>;

	watchPromises: AbsoluteFilePathMap<Promise<WatcherClose>>;

	changedFileEvent: Event<
		{
			path: AbsoluteFilePath;
			oldStats: undefined | Stats;
			newStats: Stats;
		},
		void
	>;
	deletedFileEvent: Event<AbsoluteFilePath, void>;

	// Used to maintain fake mtimes for file buffers
	buffers: AbsoluteFilePathMap<Stats>;

	init() {}

	addBuffer(path: AbsoluteFilePath, content: string) {
		this.buffers.set(
			path,
			{
				type: "file",
				size: content.length,
				mtime: Date.now(),
			},
		);
	}

	clearBuffer(path: AbsoluteFilePath) {
		this.buffers.delete(path);
	}

	unwatch(dirPath: AbsoluteFilePath) {
		const watcher = this.watchers.get(dirPath);
		if (watcher === undefined) {
			return;
		}

		this.watchers.delete(dirPath);
		watcher.close();

		// Go through and clear all files and directories from our internal maps
		// NOTE: We deliberately do not call 'deletedFileEvent' as the code that
		// calls us will already be cleaning up
		let queue: Array<AbsoluteFilePath> = [dirPath];
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

	unwatchAll() {
		for (const {close} of this.watchers.values()) {
			close();
		}
	}

	readdir(path: AbsoluteFilePath): Iterable<AbsoluteFilePath> {
		const listing = this.directoryListings.get(path);
		if (listing === undefined) {
			return [];
		} else {
			return listing.values();
		}
	}

	isDirectory(path: AbsoluteFilePath): boolean {
		return this.directories.has(path);
	}

	isFile(path: AbsoluteFilePath): boolean {
		return this.files.has(path);
	}

	getFiles(): Array<Stats> {
		return Array.from(this.files.values());
	}

	getManifestDefinition(
		dirname: AbsoluteFilePath,
	): undefined | ManifestDefinition {
		return this.manifests.get(dirname);
	}

	getManifest(dirname: AbsoluteFilePath): undefined | Manifest {
		const def = this.getManifestDefinition(dirname);
		if (def === undefined) {
			return undefined;
		} else {
			return def.manifest;
		}
	}

	getOwnedManifest(path: AbsoluteFilePath): undefined | ManifestDefinition {
		for (const dir of path.getChain()) {
			const def = this.server.memoryFs.getManifestDefinition(dir);
			if (def !== undefined) {
				return def;
			}
		}
		return undefined;
	}

	getPartialManifest(def: ManifestDefinition): WorkerPartialManifest {
		return {
			path: def.path.join(),
			type: def.manifest.type,
		};
	}

	addFileToDirectoryListing(path: AbsoluteFilePath): void {
		const dirname = path.getParent();
		let listing = this.directoryListings.get(dirname);
		if (listing === undefined) {
			listing = new AbsoluteFilePathMap();
			this.directoryListings.set(dirname, listing);
		}
		listing.set(path, path);
	}

	async handleDeletion(path: AbsoluteFilePath): Promise<void> {
		// If a folder then evict all children
		const folderInfo = this.directories.get(path);
		if (folderInfo !== undefined) {
			this.directories.delete(path);

			const listing = this.directoryListings.get(path);
			if (listing !== undefined) {
				this.directoryListings.delete(path);
				for (const path of listing.values()) {
					await this.handleDeletion(path);
				}
			}
		}

		// Wait for any subscribers that might need the file's stats
		this.server.logger.info(`[MemoryFileSystem] File deleted:`, path.toMarkup());

		// Only emit these events for files
		if (folderInfo === undefined) {
			await this.deletedFileEvent.call(path);
			this.server.refreshFileEvent.send(path);
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

	handleDeletedManifest(path: AbsoluteFilePath): void {
		const folder = path.getParent();
		const def = this.manifests.get(folder);
		if (def !== undefined) {
			this.manifests.delete(folder);
		}
	}

	async waitIfInitializingWatch(
		projectFolderPath: AbsoluteFilePath,
	): Promise<void> {
		// Defer if we're initializing a parent folder
		for (const [path, promise] of this.watchPromises) {
			if (projectFolderPath.isRelativeTo(path)) {
				await promise;
				return;
			}
		}

		// Wait if we're initializing descendents
		for (const [path, promise] of this.watchPromises) {
			if (path.isRelativeTo(projectFolderPath)) {
				await promise;
			}
		}
	}

	async watch(projectFolder: AbsoluteFilePath): Promise<void> {
		const {logger} = this.server;
		const projectFolderJoined = projectFolder.join();
		const folderLink = markup`<filelink target="${projectFolderJoined}" />`;

		// Defer if we're already currently initializing this project
		const cached = this.watchPromises.get(projectFolder);
		if (cached !== undefined) {
			await cached;
			return undefined;
		}

		// Check if we're already watching this folder
		if (this.watchers.has(projectFolder)) {
			return undefined;
		}

		// Check if we're already watching a parent directory
		for (const {path} of this.watchers.values()) {
			if (projectFolder.isRelativeTo(path)) {
				logger.info(
					`[MemoryFileSystem] Skipped crawl for ${folderLink} because we're already watching the parent directory ${path.join()}`,
				);
				return undefined;
			}
		}

		// Wait for other initializations
		await this.waitIfInitializingWatch(projectFolder);

		// New watch target
		logger.info(`[MemoryFileSystem] Adding new project folder ${folderLink}`);

		// Remove watchers that are descedents of this folder as this watcher will handle them
		for (const [loc, {close, path}] of this.watchers) {
			if (path.isRelativeTo(projectFolder)) {
				this.watchers.delete(loc);
				close();
			}
		}

		const diagnostics = this.server.createDiagnosticsProcessor({
			origins: [
				{
					category: "memory-fs",
					message: "Crawling project folder",
				},
			],
		});

		logger.info(`[MemoryFileSystem] Watching ${folderLink}`);
		const promise = createWatcher(this, diagnostics, projectFolder);
		this.watchPromises.set(projectFolder, promise);

		const watcherClose = await promise;
		this.watchers.set(
			projectFolder,
			{
				path: projectFolder,
				close: watcherClose,
			},
		);
		this.watchPromises.delete(projectFolder);

		diagnostics.maybeThrowDiagnosticsError();
	}

	// Query actual file system for stats
	async hardStat(path: AbsoluteFilePath): Promise<Stats> {
		const stats = await lstat(path);

		let type: StatsType = "unknown";
		if (stats.isDirectory()) {
			type = "directory";
		} else if (stats.isFile()) {
			type = "file";
		}

		return {
			type,
			size: stats.size,
			mtime: stats.mtimeMs,
		};
	}

	maybeGetMtime(path: AbsoluteFilePath): undefined | number {
		const stats = this.buffers.get(path) || this.files.get(path);
		if (stats === undefined) {
			return undefined;
		} else {
			return stats.mtime;
		}
	}

	getMtime(path: AbsoluteFilePath): number {
		const mtime = this.maybeGetMtime(path);
		if (mtime === undefined) {
			throw new FileNotFound(path, "Not found in memory file system");
		} else {
			return mtime;
		}
	}

	getFileStats(path: AbsoluteFilePath): undefined | Stats {
		return this.files.get(path);
	}

	getFileStatsAssert(path: AbsoluteFilePath): Stats {
		const stats = this.getFileStats(path);
		if (stats === undefined) {
			throw new FileNotFound(path, "Not found in memory file system");
		}
		return stats;
	}

	isIgnored(path: AbsoluteFilePath, type: "directory" | "file"): boolean {
		const project = this.server.projectManager.findProjectExisting(path);
		if (project === undefined) {
			return false;
		}

		// If we're a file and don't have an extension handler so there's no reason for us to care about it
		if (type === "file" && getFileHandler(path, project.config) === undefined) {
			return true;
		}

		// Ensure we aren't in any of the default denylists
		const basename = path.getBasename();
		if (DEFAULT_DENYLIST.includes(basename)) {
			return true;
		}

		return false;
	}

	isInsideProject(path: AbsoluteFilePath): boolean {
		return path.getSegments().includes("node_modules") === false;
	}

	// This is a wrapper around _declareManifest as it can produce diagnostics
	async declareManifest(opts: DeclareManifestOpts): Promise<void> {
		const {diagnostics} = await catchDiagnostics(() => {
			return this._declareManifest(opts);
		});

		if (diagnostics !== undefined) {
			opts.diagnostics.addDiagnostics(diagnostics);
		}
	}

	async _declareManifest(
		{
			path,
			diagnostics,
		}: DeclareManifestOpts,
	): Promise<void> {
		// Fetch the manifest
		const manifestRaw = await readFileText(path);
		const hash = crypto.createHash("sha256").update(manifestRaw).digest("hex");

		const consumer = consumeJSON({
			path,
			input: manifestRaw,
			consumeDiagnosticCategory: "parse/manifest",
		});

		const {
			manifest,
			diagnostics: normalizedDiagnostics,
		} = await normalizeManifest(path, consumer);

		// If manifest is undefined then we failed to validate and have diagnostics
		if (normalizedDiagnostics.length > 0) {
			diagnostics.addDiagnostics(normalizedDiagnostics);
			return;
		}

		const folder = path.getParent();
		const manifestId = this.manifestCounter++;
		const def: ManifestDefinition = {
			id: manifestId,
			path,
			folder,
			consumer,
			manifest,
			hash,
		};

		this.manifests.set(folder, def);

		// If we aren't in node_modules then this is a project package
		const isProjectPackage = this.isInsideProject(path);
		const {projectManager} = this.server;

		if (isProjectPackage && consumer.has(ROME_CONFIG_PACKAGE_JSON_FIELD)) {
			await projectManager.addDiskProject({
				projectFolder: folder,
				configPath: path,
			});
		}

		const project = projectManager.assertProjectExisting(path);
		projectManager.declareManifest(project, isProjectPackage, def, diagnostics);

		// Tell all workers of our discovery
		for (const worker of this.server.workerManager.getWorkers()) {
			worker.bridge.updateManifests.call({
				manifests: [{id: def.id, manifest: this.getPartialManifest(def)}],
			});
		}
	}

	glob(
		cwd: AbsoluteFilePath,
		opts: MemoryFSGlobOptions = {},
	): AbsoluteFilePathSet {
		const {extensions, configCategory, test, overrideIgnore = []} = opts;

		const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		let crawl: Array<AbsoluteFilePath> = [cwd];

		const ignoresByProject: Map<ProjectDefinition, PathPatterns> = new Map();

		while (crawl.length > 0) {
			const path = crawl.pop()!;

			// `cache: false` to allow calling us with deleted paths
			const project = this.server.projectManager.findProjectExisting(path);

			let ignore: PathPatterns = overrideIgnore;

			// Get ignore patterns
			if (configCategory !== undefined && project !== undefined) {
				const projectIgnore = ignoresByProject.get(project);
				if (projectIgnore === undefined) {
					ignore = concatGlobIgnore([
						...ignore,
						...project.config[configCategory].ignore,
					]);
					ignoresByProject.set(project, ignore);
				} else {
					ignore = projectIgnore;
				}
			}

			const ignoreMatched = matchPathPatterns(path, ignore, cwd);

			// Don't even recurse into explicit matches
			if (ignoreMatched === "EXPLICIT_MATCH") {
				continue;
			}

			// Add if a matching file
			if (this.files.has(path) && ignoreMatched === "NO_MATCH") {
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

				paths.add(path);
				continue;
			}

			// Crawl if we're a folder
			// NOTE: We still continue crawling on implicit matches
			const listing = this.directoryListings.get(path);
			if (listing !== undefined) {
				crawl = crawl.concat(Array.from(listing.values()));
				continue;
			}

			// TODO maybe throw? not a file or folder, doesn't exist!
		}

		return paths;
	}

	getAllFilesInFolder(folder: AbsoluteFilePath): Array<AbsoluteFilePath> {
		let files: Array<AbsoluteFilePath> = [];

		const listing = this.directoryListings.get(folder);
		if (listing !== undefined) {
			for (const file of listing.keys()) {
				if (this.files.has(file)) {
					files.push(file);
				} else {
					files = files.concat(this.getAllFilesInFolder(file));
				}
			}
		}

		return files;
	}

	countFiles(folder: AbsoluteFilePath): number {
		let count: number = 0;

		const listing = this.directoryListings.get(folder);
		if (listing !== undefined) {
			for (const file of listing.keys()) {
				count++;
				count += this.countFiles(file);
			}
		}

		return count;
	}

	hasStatsChanged(path: AbsoluteFilePath, newStats: Stats): boolean {
		const oldStats = this.directories.get(path) || this.files.get(path);
		return oldStats === undefined || newStats.mtime !== oldStats.mtime;
	}

	async addDirectory(
		folderPath: AbsoluteFilePath,
		stats: Stats,
		opts: CrawlOptions,
	): Promise<boolean> {
		if (!this.hasStatsChanged(folderPath, stats)) {
			return false;
		}

		// Check if this folder has been ignored
		if (this.isIgnored(folderPath, "directory")) {
			return false;
		}

		if (opts.tick !== undefined) {
			opts.tick(folderPath);
		}

		this.addFileToDirectoryListing(folderPath);
		this.directories.set(folderPath, stats);

		if (opts.onFoundDirectory !== undefined) {
			opts.onFoundDirectory(folderPath);
		}

		// Crawl the folder
		const files = await readDirectory(folderPath);

		// Declare the file
		const declareItem = async (path: AbsoluteFilePath) => {
			const stats = await this.hardStat(path);
			if (stats.type === "file") {
				await this.addFile(path, stats, opts);
			} else if (stats.type === "directory") {
				await this.addDirectory(path, stats, opts);
			}
		};

		// Give priority to package.json in case we want to derive something from the project config
		for (const priorityBasename of PRIORITY_FILES) {
			for (const file of files) {
				if (priorityBasename === file.getBasename()) {
					files.delete(file);
					await declareItem(file);
				}
			}
		}

		// Add the rest of the items
		await Promise.all(Array.from(files, declareItem));

		return true;
	}

	exists(path: AbsoluteFilePath): undefined | boolean {
		// if we have this in our cache then the file exists
		if (this.files.has(path) || this.directories.has(path)) {
			return true;
		}

		// If we're still performing an initial crawl of any path higher in the tree then we don't know if it exists yet
		for (const projectFolder of this.watchPromises.keys()) {
			if (path.isRelativeTo(projectFolder)) {
				return undefined;
			}
		}

		// if we're watching the parent folder then we'd have it in our cache if it existed
		const parent = path.getParent();
		if (this.directories.has(parent)) {
			return false;
		}

		return undefined;
	}

	async existsHard(path: AbsoluteFilePath): Promise<boolean> {
		const resolvedExistence: undefined | boolean = this.exists(path);
		if (resolvedExistence === undefined) {
			return exists(path);
		} else {
			return resolvedExistence;
		}
	}

	async addFile(
		path: AbsoluteFilePath,
		stats: Stats,
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

		this.files.set(path, stats);
		this.addFileToDirectoryListing(path);

		const basename = path.getBasename();
		const dirname = path.getParent();

		// Warn about potentially incorrect Rome config filenames
		const {projectManager} = this.server;
		projectManager.checkConfigFile(path, opts.diagnostics);

		// Add project if this is a config
		if (ROME_CONFIG_FILENAMES.includes(basename)) {
			await projectManager.addDiskProject({
				projectFolder: dirname,
				configPath: path,
			});
		}

		if (isValidManifest(path)) {
			await this.declareManifest({
				diagnostics: opts.diagnostics,
				dirname,
				path,
			});
		}

		// Detect file changes
		const oldStats = this.getFileStats(path);
		if (oldStats !== undefined && opts.reason === "watch") {
			this.server.logger.info(
				`[MemoryFileSystem] File change:`,
				path.toMarkup(),
			);
			this.server.refreshFileEvent.send(path);
			this.changedFileEvent.send({path, oldStats, newStats: stats});
		}

		return true;
	}
}
