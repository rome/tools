import {Server, ServerRequest} from "@internal/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
} from "@internal/path";
import {FSHandle, FSStats} from "@internal/fs";
import {Dict} from "@internal/typescript-helpers";
import {json} from "@internal/codec-config";
import {
	Diagnostic,
	DiagnosticLocation,
	catchDiagnostics,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {ReporterNamespace} from "@internal/cli-reporter";
import {createResourceFromTimeout} from "@internal/resources";
import {promiseAllFrom} from "@internal/async";

export type RecoverySaveFile =
	| {
			type: "UNSAFE_WRITE";
			content: string;
		}
	| {
			type: "WRITE";
			mtimeNs: undefined | bigint;
			content: string;
		};

type MemoryStore = {
	storeId: string;
	requestId: number;
	fileCounter: number;
	index: {
		timestamp: string;
		command: string;
		files: Dict<string>;
	};
};

export type RecoveryDiskStore = {
	storeId: string;
	timestamp: string;
	command: string;
	entries: DiskStoreEntry[];
};

type DiskStoreEntry = {
	fileId: string;
	artifactPath: AbsoluteFilePath;
	originalPath: AbsoluteFilePath;
};

type WriteFilesEvents = {
	onFileDone: (path: AbsoluteFilePath) => void;
	beforeFileWrite: (
		path: AbsoluteFilePath,
		fh: FSHandle,
	) => void | Promise<void>;
	expectedExists: (path: AbsoluteFilePath) => void;
	unexpectedExists: (path: AbsoluteFilePath) => void;
	unexpectedModified: (
		path: AbsoluteFilePath,
		expectedMtime: bigint,
		actualMtime: bigint,
	) => void;
};

type WriteFilesOptions = {
	unsafeWrites: boolean;
};

const DEFAULT_WRITE_FILES_EVENTS: WriteFilesEvents = {
	onFileDone() {},
	beforeFileWrite() {},
	unexpectedModified: (path, expectedMtime, actualMtime) => {
		throw new Error(
			`File ${path.join()} was not updated as it was changed since we read it`,
		);
	},
	expectedExists: (path) => {
		throw new Error(
			`File ${path.join()} was not updated as it does not exist when we expected it to`,
		);
	},
	unexpectedExists: (path) => {
		throw new Error(
			`File ${path.join()} was not written as it exists when we didn't expect it`,
		);
	},
};

// Chosen arbitrarily
const MAX_STORE_ENTRIES = 5;

// This class is used for saving files before we modify them. This is to protect users from
// data loss when running commands.
// We are deliberately careful here to avoid race conditions and properly clear and restore.
export default class RecoveryStore {
	constructor(server: Server) {
		this.server = server;
		this.requestIdToStore = new Map();
		this.evictableStoreIds = [];
		this.blockSave = undefined;
		this.logger = server.logger.namespace(markup`RecoveryStore`);
		this.shouldTruncate = true;
		this.recoveryDirectoryPath = server.userConfig.recoveryPath;
	}

	private recoveryDirectoryPath: AbsoluteFilePath;
	private requestIdToStore: Map<number, MemoryStore>;
	private blockSave: undefined | Promise<unknown>;
	private evictableStoreIds: string[];
	private server: Server;
	private logger: ReporterNamespace;
	private shouldTruncate: boolean;

	public getDirectory(): AbsoluteFilePath {
		return this.recoveryDirectoryPath;
	}

	private getStoreDirectoryPath(storeId: string): AbsoluteFilePath {
		return this.recoveryDirectoryPath.append(storeId);
	}

	private getStoreIndexPath(storeId: string): AbsoluteFilePath {
		return this.getStoreDirectoryPath(storeId).append("index.json");
	}

	private async readRecoveryDirectory(): Promise<AbsoluteFilePathSet> {
		const paths: [AbsoluteFilePath, number][] = [];

		for (const path of await this.recoveryDirectoryPath.readDirectory()) {
			const basename = path.getBasename();
			if (basename[0] === ".") {
				continue;
			}

			// First segment is a unix epoch
			const timestamp = new Date(Number(basename.split("-")[0])).valueOf();
			if (isNaN(timestamp)) {
				// Malformed
				continue;
			}

			paths.push([path, timestamp]);
		}

		paths.sort((a, b) => b[1] - a[1]);

		return new AbsoluteFilePathSet(paths.map(([path]) => path));
	}

	public async init() {
		await this.recoveryDirectoryPath.createDirectory();

		// Register initial stores
		this.evictableStoreIds = Array.from(
			await this.readRecoveryDirectory(),
			(path) => path.getBasename(),
		);
		this.logger.info(
			markup`Initial store content ${prettyFormat(this.evictableStoreIds)}`,
		);

		// Drop excessive directories
		await this.truncate();
	}

	public async clear() {
		this.evictableStoreIds = [];
		this.requestIdToStore.clear();

		for (const path of await this.readRecoveryDirectory()) {
			await this.drop(path.getBasename(), "Clear requested");
		}
	}

	// Drop old stores if we are at max entries
	private async truncate() {
		if (!this.shouldTruncate) {
			return;
		}

		while (this.evictableStoreIds.length > MAX_STORE_ENTRIES) {
			const dropStoreId = this.evictableStoreIds.pop()!;
			await this.drop(dropStoreId, "Reached capacity");
		}
	}

	private async drop(storeId: string, reason: string) {
		this.logger.info(
			markup`Dropping recovery store <emphasis>${storeId}</emphasis>. Reason: ${reason}`,
		);
		await this.getStoreDirectoryPath(storeId).removeDirectory();
	}

	public async getAllStores(): Promise<{
		diagnostics: Diagnostic[];
		stores: RecoveryDiskStore[];
	}> {
		const stores: RecoveryDiskStore[] = [];
		let diagnostics: Diagnostic[] = [];

		for (const path of await this.readRecoveryDirectory()) {
			const {diagnostics: storeDiagnostics} = await catchDiagnostics(async () => {
				const store = await this.maybeGetStore(path.getBasename());
				if (store !== undefined) {
					stores.push(store);
				}
			});
			if (storeDiagnostics !== undefined) {
				diagnostics = diagnostics.concat(storeDiagnostics);
			}
		}

		return {stores, diagnostics};
	}

	public async getStore(
		storeId: string,
		location?: DiagnosticLocation,
	): Promise<RecoveryDiskStore> {
		const store = await this.maybeGetStore(storeId);
		if (store === undefined) {
			if (location === undefined) {
				throw new Error(`Recovery store ${storeId} not found`);
			} else {
				throw createSingleDiagnosticsError({
					description: descriptions.RECOVERY_STORE.NOT_FOUND(storeId),
					location,
				});
			}
		} else {
			return store;
		}
	}

	private async maybeGetStore(
		storeId: string,
	): Promise<undefined | RecoveryDiskStore> {
		const indexPath = this.getStoreIndexPath(storeId);
		if (await indexPath.notExists()) {
			return undefined;
		}

		const indexContent = await indexPath.readFileText();
		const index = json.consumeValue({
			input: indexContent,
			path: indexPath,
		});

		const entries: DiskStoreEntry[] = [];

		for (const [key, value] of index.get("files").asMap()) {
			entries.push({
				fileId: key,
				artifactPath: this.getStoreDirectoryPath(storeId).append(key),
				originalPath: value.asAbsoluteFilePath(),
			});
		}

		return {
			storeId,
			timestamp: index.get("timestamp").asString(),
			command: index.get("command").asString(),
			entries,
		};
	}

	private async createRequestStore(req: ServerRequest): Promise<MemoryStore> {
		await this.truncate();
		const timestamp = new Date().toISOString();
		const storeId = `${Date.now()}-${req.query.commandName}-${req.id}`;
		const command = req.getDiagnosticLocationFromFlags("none").sourceText;
		const store: MemoryStore = {
			index: {
				timestamp,
				command,
				files: {},
			},
			fileCounter: 0,
			storeId,
			requestId: req.id,
		};
		this.requestIdToStore.set(req.id, store);

		const path = this.getStoreDirectoryPath(store.storeId);
		await path.createDirectory();
		this.logger.info(
			markup`Created store <emphasis>${store.storeId}</emphasis> at <emphasis>${path}</emphasis>`,
		);

		// Only consider a request up for eviction when the request has finished
		req.endEvent.subscribe(async () => {
			await this.commit(req);
			this.evictableStoreIds.unshift(storeId);
		});

		return store;
	}

	public async save(
		req: ServerRequest,
		path: AbsoluteFilePath,
		content: ArrayBufferView,
	) {
		if (this.blockSave !== undefined) {
			await this.blockSave;
		}

		let store = this.requestIdToStore.get(req.id);
		if (store === undefined) {
			const promise = this.createRequestStore(req);
			this.blockSave = promise;
			store = await promise;
		}

		const fileId = String(store.fileCounter++);
		store.index.files[fileId] = path.join();

		const storePath = this.getStoreDirectoryPath(store.storeId).append(fileId);
		await storePath.writeFile(content);
		this.logger.info(
			markup`Save file from <emphasis>${path}</emphasis> to <emphasis>${storePath}</emphasis>`,
		);
	}

	// Take the contents of the store and write the artifacts back to their original location
	public async apply(
		req: ServerRequest,
		storeId: string,
		location?: DiagnosticLocation,
		filter?: (store: RecoveryDiskStore) => Promise<undefined | (string[])>,
	): Promise<DiskStoreEntry[]> {
		const store = await this.getStore(storeId, location);

		let entries: DiskStoreEntry[] = [];

		let fileIdsAllowlist: undefined | (string[]);
		if (filter !== undefined) {
			fileIdsAllowlist = await filter(store);
		}

		for (const entry of store.entries) {
			const {fileId, artifactPath, originalPath} = entry;
			if (fileIdsAllowlist !== undefined && !fileIdsAllowlist.includes(fileId)) {
				continue;
			}

			// Calculate mtime we expect
			let mtimeNs: undefined | bigint;
			if (await originalPath.exists()) {
				mtimeNs = (await originalPath.lstat()).mtimeNs;
			}

			const content = await artifactPath.readFileText();

			req.queueSaveFile(
				originalPath,
				{
					type: "WRITE",
					mtimeNs,
					content,
				},
			);
			entries.push(entry);
		}

		try {
			// Pause truncation since declaring this new store could cause the one we're applying to overflow
			this.shouldTruncate = false;
			await req.flushFiles();
			await this.drop(storeId, "Applied patch already");
		} finally {
			this.shouldTruncate = true;
		}

		return entries;
	}

	// Commits the index file that we use to map the artifacts to original paths
	public async commit(req: ServerRequest) {
		const store = this.requestIdToStore.get(req.id);
		if (store !== undefined) {
			const indexPath = this.getStoreIndexPath(store.storeId);
			await indexPath.writeFile(json.stringify(store.index));
			this.logger.info(
				markup`Committed store index to <emphasis>${indexPath}</emphasis>`,
			);
		}
	}

	async writeFile(
		path: AbsoluteFilePath,
		op: RecoverySaveFile,
		events: WriteFilesEvents,
		registerFile: (paths: AbsoluteFilePath[]) => void,
	): Promise<boolean> {
		const {server} = this;
		let fd: undefined | FSHandle;
		let success = false;

		try {
			if (op.type === "UNSAFE_WRITE") {
				await path.writeFile(op.content);
				success = true;
			} else if (op.type === "WRITE") {
				const {mtimeNs, content} = op;

				if (mtimeNs === undefined) {
					const {content} = op;
					try {
						// `mtime === undefined` means we expect the file to not exist
						// wx: Open file for writing. Fails if the path exists.
						fd = await path.openFile("wx");
						await fd.writeFile(content);
						success = true;
					} catch (err) {
						if (err.code === "EEXIST") {
							events.unexpectedExists(path);
						} else {
							throw err;
						}
					}
				} else {
					try {
						// `mtime !== undefined` means we expect the file to exist
						// r+: Open file for reading and writing. An exception occurs if the file does not exist.
						fd = await path.openFile("r+");

						// First verify the mtime
						// @ts-ignore: This is accurate
						const stats: FSStats = await fd.stat({bigint: true});
						if (stats.mtimeNs === mtimeNs) {
							await events.beforeFileWrite(path, fd);
							await fd.truncate(0);
							await fd.write(content, 0);
							success = true;
						} else {
							registerFile([path]);
							events.unexpectedModified(path, mtimeNs, stats.mtimeNs);
						}
					} catch (err) {
						if (err.code === "ENOENT") {
							events.expectedExists(path);
						} else {
							throw err;
						}
					}
				}
			}
		} catch (err) {
			throw err;
		} finally {
			// Close file descriptor
			if (fd !== undefined) {
				await fd.close();
			}

			// We want writeFiles to only return once all the refreshFileEvent handlers have ran
			// We call refreshPath to do a hard check on the filesystem and update our in memory fs
			// This mitigates slow watch events
			server.fatalErrorHandler.wrapPromise(
				server.memoryFs.refreshPath(path, {}, "Server.writeFiles"),
			);
		}

		return success;
	}

	// Utility to write a list of files and wait for all refresh events to be emitted
	// We optionally validate mtime of the existing file if specified
	// The bar here for race conditions should be extremely high as we want to minimize bad writes
	public async writeFiles(
		files: AbsoluteFilePathMap<RecoverySaveFile>,
		opts: WriteFilesOptions = {unsafeWrites: false},
		events: WriteFilesEvents = DEFAULT_WRITE_FILES_EVENTS,
	): Promise<number> {
		if (files.size === 0) {
			return 0;
		}

		// For unsafe writes we don't bother checking for locks or mtime
		if (opts.unsafeWrites) {
			await promiseAllFrom(
				files,
				async ([path, {content}]) => {
					await path.writeFile(content);
				},
			);
			return files.size;
		}

		const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet(files.keys());
		const {server} = this;
		const resources = server.resources.createContainer(
			"RecoveryStore.writeFiles",
		);

		// Files successfully written
		let fileCount = 0;

		let registerFile: (paths: AbsoluteFilePath[]) => void = (paths) => {
			throw new Error("Function should have been replaced");
		};

		// refreshFileEvent doesn't resolve
		const waitRefresh: Promise<void> = new Promise((resolve) => {
			registerFile = (refreshedPaths) => {
				for (const path of refreshedPaths) {
					if (!paths.has(path)) {
						continue;
					}

					events.onFileDone(path);
					paths.delete(path);
					if (paths.size === 0) {
						resolve();
					}
				}
			};

			resources.add(
				server.refreshFileEvent.subscribe((events) => {
					for (const {path} of events) {
						registerFile([path]);
					}
				}),
			);
		});

		try {
			// Write files
			// We call fs.open to avoid race conditions since we want to check the mtime, and then update the
			// file if it's the same
			await promiseAllFrom(
				files,
				async ([path, op]) => {
					const success = await this.writeFile(path, op, events, registerFile);
					if (success) {
						fileCount++;
					}
				},
			);

			// Protects against file events not being emitted and causing hanging
			const timeoutPromise = new Promise((resolve, reject) => {
				resources.add(
					createResourceFromTimeout(
						"FileHangDetector",
						setTimeout(
							() => {
								const lines = [
									"File events should have been emitted within a second. Did not receive an event for:",
								];
								for (const path of paths) {
									lines.push(` - ${path.join()}`);
								}
								reject(new Error(lines.join("\n")));
							},
							1_000,
						),
					),
				);
			});

			await Promise.race([waitRefresh, timeoutPromise]);
		} finally {
			await resources.release();
			await this.server.memoryFs.processingLock.wait();
		}

		return fileCount;
	}
}
