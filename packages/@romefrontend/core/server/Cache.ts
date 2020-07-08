/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	WorkerAnalyzeDependencyResult,
	WorkerCompileResult,
	WorkerLintResult,
} from "../common/bridges/WorkerBridge";
import {ModuleSignature} from "@romefrontend/js-analysis";
import Server from "./Server";
import {ProjectDefinition} from "@romefrontend/project";
import {VERSION} from "../common/constants";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@romefrontend/path";
import {
	createDirectory,
	readFileText,
	removeFile,
	writeFile,
} from "@romefrontend/fs";
import {stringifyJSON} from "@romefrontend/codec-json";

export type CacheEntry = {
	version: string;
	configHash: string;
	projectDir: string;
	mtime: number;
	compile: {
		[key: string]: WorkerCompileResult;
	};
	lint: {
		[key: string]: WorkerLintResult;
	};
	analyzeDependencies: undefined | WorkerAnalyzeDependencyResult;
	moduleSignature: undefined | ModuleSignature;
};

// Basic checks to determine if we can consider a and b to be mergable
function areEntriesEqual(a: CacheEntry, b: CacheEntry): boolean {
	if (a.version !== b.version) {
		// Outdated cache file
		return false;
	}

	if (a.configHash !== b.configHash) {
		// Project config has been changed since this was last updated
		return false;
	}

	if (a.mtime !== b.mtime) {
		// File has been changed
		return false;
	}

	return true;
}

// Write cache entries every 5 seconds after the first modification
const BATCH_WRITES_MS = 5_000;

export default class Cache {
	constructor(server: Server) {
		this.server = server;
		this.loadedEntries = new AbsoluteFilePathMap();
		this.disabled =
			!server.options.forceCacheEnabled &&
			(process.env.ROME_CACHE === "0" || process.env.ROME_DEV === "1");
		this.cachePath = server.userConfig.cachePath;

		this.runningWritePromise = undefined;
		this.pendingWriteTimer = undefined;
		this.pendingWrites = new AbsoluteFilePathMap();
	}

	disabled: boolean;
	loadedEntries: AbsoluteFilePathMap<CacheEntry>;
	server: Server;
	cachePath: AbsoluteFilePath;

	runningWritePromise: undefined | Promise<void>;
	pendingWrites: AbsoluteFilePathMap<CacheEntry>;
	pendingWriteTimer: undefined | NodeJS.Timeout;

	async init() {
		this.server.memoryFs.deletedFileEvent.subscribe((filename) => {
			return this.server.cache.handleDeleted(filename);
		});

		const {memoryFs} = this.server;
		await createDirectory(this.cachePath);
		await memoryFs.watch(this.cachePath);

		this.server.endEvent.subscribe(async () => {
			// Wait on possible running writePending
			await this.runningWritePromise;

			// Write any remaining
			await this.writePending("end");
		});
	}

	async writePending(reason: "queue" | "end") {
		// Clear timer since we're now running
		const {pendingWriteTimer} = this;
		if (pendingWriteTimer !== undefined) {
			clearTimeout(pendingWriteTimer);
		}

		const {pendingWrites} = this;
		this.pendingWrites = new AbsoluteFilePathMap();

		// Write pending files
		const filelinks: Array<string> = [];
		for (const [path, entry] of pendingWrites) {
			filelinks.push(path.toMarkup());
			await writeFile(path, stringifyJSON(entry));
		}

		// Log
		const {logger} = this.server;
		if (filelinks.length > 0) {
			logger.info(`[Cache] Wrote entries due to ${reason}`);
			logger.list(filelinks);
		}
	}

	addPendingWrite(path: AbsoluteFilePath, entry: CacheEntry) {
		this.pendingWrites.set(path, entry);

		// Set a write timer
		const {pendingWriteTimer} = this;
		if (pendingWriteTimer !== undefined) {
			return;
		}

		this.pendingWriteTimer = setTimeout(
			() => {
				this.runningWritePromise = this.writePending("queue").catch((err) => {
					this.server.onFatalError(err);
				}).finally(() => {
					// Finished running
					this.runningWritePromise = undefined;
				});
			},
			BATCH_WRITES_MS,
		);
	}

	async createEmptyEntry(path: AbsoluteFilePath): Promise<CacheEntry> {
		const {projectManager, memoryFs} = this.server;

		const project: ProjectDefinition = await projectManager.assertProject(path);

		const configHashes = [...project.meta.configHashes];
		const pkg = this.server.memoryFs.getOwnedManifest(path);
		if (pkg !== undefined) {
			configHashes.push(pkg.hash);
		}

		const entry: CacheEntry = {
			version: VERSION,
			projectDir: project.folder.join(),
			configHash: configHashes.join(";"),
			mtime: memoryFs.getMtime(path),
			compile: {},
			analyzeDependencies: undefined,
			moduleSignature: undefined,
			lint: {},
		};

		return entry;
	}

	getCacheFilename(path: AbsoluteFilePath): AbsoluteFilePath {
		const uid = this.server.projectManager.getUid(path, true);
		// We add a single underscore to prevent the extension from being registered
		return this.cachePath.append(`${uid}_`);
	}

	async handleDeleted(path: AbsoluteFilePath) {
		// Handle the file not existing
		const cacheFilename = this.getCacheFilename(path);
		await removeFile(cacheFilename);
		this.loadedEntries.delete(path);
	}

	async get(path: AbsoluteFilePath): Promise<CacheEntry> {
		const emptyEntry = await this.createEmptyEntry(path);

		// If we have a loaded memory entry, make sure it's valid compared to the default entry (file changes etc)
		let loaded = this.loadedEntries.get(path);
		if (loaded !== undefined && areEntriesEqual(loaded, emptyEntry)) {
			return loaded;
		}

		if (this.disabled) {
			return emptyEntry;
		}

		const cacheFilename = this.getCacheFilename(path);
		const entry = await this.checkPossibleDiskCacheEntry(
			cacheFilename,
			emptyEntry,
		);
		this.loadedEntries.set(path, entry);
		return entry;
	}

	async checkPossibleDiskCacheEntry(
		cacheFilename: AbsoluteFilePath,
		emptyEntry: CacheEntry,
	): Promise<CacheEntry> {
		const {memoryFs} = this.server;

		if (!memoryFs.exists(cacheFilename)) {
			return emptyEntry;
		}

		try {
			const json = await readFileText(cacheFilename);
			const obj = JSON.parse(json);

			if (areEntriesEqual(emptyEntry, obj)) {
				return {...emptyEntry, ...obj};
			} else {
				// If the entries aren't equal then there's something wrong with the cache entry
				await this.handleDeleted(cacheFilename);
				return emptyEntry;
			}
		} catch (err) {
			// TODO add some heuristic to only catch json and cache permission errors
			return emptyEntry;
		}
	}

	async update(
		path: AbsoluteFilePath,
		partialEntryCallback:
			| Partial<CacheEntry>
			| ((entry: CacheEntry) => Partial<CacheEntry>),
	) {
		const currEntry = await this.get(path);
		const partialEntry: Partial<CacheEntry> =
			typeof partialEntryCallback === "function"
				? partialEntryCallback(currEntry)
				: partialEntryCallback;

		const entry: CacheEntry = {
			...currEntry,
			...partialEntry,
		};

		// TODO should batch these and write during idle time
		const cacheFilename = this.getCacheFilename(path);
		this.loadedEntries.set(path, entry);

		if (this.disabled) {
			return;
		}

		await createDirectory(cacheFilename.getParent());
		this.addPendingWrite(cacheFilename, entry);
	}
}
