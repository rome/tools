import {ReporterNamespace} from "@internal/cli-reporter";
import {
	RSERValue,
	encodeValueToRSERSingleMessageStream,
} from "@internal/codec-binary-serial";
import {
	createDirectory,
	exists,
	removeDirectory,
	removeFile,
	writeFile,
} from "@internal/fs";
import {AnyMarkups, markup} from "@internal/markup";
import {AbsoluteFilePath, AbsoluteFilePathMap, UIDPath} from "@internal/path";
import FatalErrorHandler from "./FatalErrorHandler";
import {UserConfig} from "./userConfig";

// Write cache entries every 5 seconds after the first modification
const BATCH_WRITES_MS = 5_000;

type WriteOperation =
	| {
			type: "update";
			value: RSERValue;
		}
	| {
			type: "delete";
		};

export default class Cache {
	constructor(
		namespace: string,
		{fatalErrorHandler, userConfig, parentLogger, writeDisabled, readDisabled}: {
			fatalErrorHandler: FatalErrorHandler;
			userConfig: UserConfig;
			parentLogger: ReporterNamespace;
			writeDisabled: boolean;
			readDisabled: boolean;
		},
	) {
		this.writeDisabled = writeDisabled;
		this.readDisabled = readDisabled;

		this.directoryPath = userConfig.cacheDirectory.append(namespace);

		this.logger = parentLogger.namespace(markup`Cache`);
		this.fatalErrorHandler = fatalErrorHandler;
		this.runningWritePromise = undefined;
		this.pendingWriteTimer = undefined;
		this.pendingWrites = new AbsoluteFilePathMap();
	}

	public writeDisabled: boolean;
	public readDisabled: boolean;
	public logger: ReporterNamespace;

	private fatalErrorHandler: FatalErrorHandler;
	protected directoryPath: AbsoluteFilePath;

	protected runningWritePromise: undefined | Promise<void>;
	protected pendingWrites: AbsoluteFilePathMap<AbsoluteFilePathMap<WriteOperation>>;
	protected pendingWriteTimer: undefined | NodeJS.Timeout;

	public getDirectory(): AbsoluteFilePath {
		return this.directoryPath;
	}

	public async remove(uid: UIDPath, path: AbsoluteFilePath) {
		const directory = this.getCacheDirectory(uid);
		path;

		this.pendingWrites.delete(directory);

		if (await exists(directory)) {
			await removeDirectory(directory);
		}
	}

	protected getCacheDirectory(uid: UIDPath): AbsoluteFilePath {
		return this.directoryPath.append(uid.join());
	}

	public getCacheFilename(uid: UIDPath, name: string): AbsoluteFilePath {
		return this.getCacheDirectory(uid).append(`${name}.bin`);
	}

	public async teardown() {
		// Wait on possible running writePending
		await this.runningWritePromise;

		// Write any remaining
		await this.writePending("end");
	}

	protected async writePending(reason: "queue" | "end") {
		// Clear timer since we're now running
		const {pendingWriteTimer} = this;
		if (pendingWriteTimer !== undefined) {
			clearTimeout(pendingWriteTimer);
		}

		const {pendingWrites} = this;
		this.pendingWrites = new AbsoluteFilePathMap();

		// Write pending files
		const filelinks: AnyMarkups = [];
		for (const [directory, ops] of pendingWrites) {
			await createDirectory(directory);

			for (const [path, op] of ops) {
				filelinks.push(markup`${path}`);
				switch (op.type) {
					case "delete": {
						await removeFile(path);
						break;
					}

					case "update": {
						await writeFile(
							path,
							new DataView(encodeValueToRSERSingleMessageStream(op.value)),
						);
						break;
					}
				}
			}
		}

		// Log
		const {logger} = this;
		if (filelinks.length > 0) {
			logger.info(markup`Wrote entries due to ${reason}`);
			logger.list(filelinks);
		}
	}

	public addPendingWrite(path: AbsoluteFilePath, op: WriteOperation) {
		if (this.writeDisabled) {
			return;
		}

		const directory = path.getParent();
		let files = this.pendingWrites.get(directory);
		if (files === undefined) {
			files = new AbsoluteFilePathMap();
			this.pendingWrites.set(directory, files);
		}
		files.set(path, op);

		// Set a write timer
		const {pendingWriteTimer} = this;
		if (pendingWriteTimer !== undefined) {
			return;
		}

		this.pendingWriteTimer = setTimeout(
			() => {
				this.runningWritePromise = this.writePending("queue").catch((err) => {
					this.fatalErrorHandler.handle(err);
				}).finally(() => {
					// Finished running
					this.runningWritePromise = undefined;
				});
			},
			BATCH_WRITES_MS,
		);
	}
}
