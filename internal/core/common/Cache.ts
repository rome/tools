import {getEnvVar} from "@internal/cli-environment";
import {
	RSERValue,
	encodeValueToRSERBufferMessage,
} from "@internal/codec-binary-serial";
import {
	createDirectory,
	exists,
	removeDirectory,
	removeFile,
	writeFile,
} from "@internal/fs";
import {AnyMarkups, markup} from "@internal/markup";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import FatalErrorHandler from "./FatalErrorHandler";
import {UserConfig} from "./userConfig";
import Logger from "./utils/Logger";

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
		{fatalErrorHandler, userConfig, logger, forceEnabled}: {
			fatalErrorHandler: FatalErrorHandler;
			userConfig: UserConfig;
			logger: Logger;
			forceEnabled?: boolean;
		},
	) {
		let disabled = false;
		if (getEnvVar("ROME_DEV").type === "ENABLED") {
			disabled = true;
		}
		if (getEnvVar("ROME_CACHE").type === "DISABLED") {
			disabled = true;
		}
		if (forceEnabled) {
			disabled = false;
		}
		this.disabled = disabled;

		this.cachePath = userConfig.cachePath.append(namespace);
		this.logger = logger;
		this.fatalErrorHandler = fatalErrorHandler;
		this.runningWritePromise = undefined;
		this.pendingWriteTimer = undefined;
		this.pendingWrites = new AbsoluteFilePathMap();
	}

	private fatalErrorHandler: FatalErrorHandler;

	public disabled: boolean;
	protected logger: Logger;
	protected cachePath: AbsoluteFilePath;
	protected runningWritePromise: undefined | Promise<void>;
	protected pendingWrites: AbsoluteFilePathMap<AbsoluteFilePathMap<WriteOperation>>;
	protected pendingWriteTimer: undefined | NodeJS.Timeout;

	public getDirectory(): AbsoluteFilePath {
		return this.cachePath;
	}

	public async remove(uid: string, path: AbsoluteFilePath) {
		const directory = this.getCacheDirectory(uid);
		path;

		this.pendingWrites.delete(directory);

		if (await exists(directory)) {
			await removeDirectory(directory);
		}
	}

	protected getCacheDirectory(uid: string): AbsoluteFilePath {
		return this.cachePath.append(uid);
	}

	public getCacheFilename(uid: string, name: string): AbsoluteFilePath {
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
		for (const [directory, files] of pendingWrites) {
			await createDirectory(directory);

			for (const [path, op] of files) {
				filelinks.push(markup`${path}`);
				switch (op.type) {
					case "delete": {
						await removeFile(path);
						break;
					}

					case "update": {
						await writeFile(
							path,
							new DataView(encodeValueToRSERBufferMessage(op.value)),
						);
						break;
					}
				}
			}
		}

		// Log
		const {logger} = this;
		if (filelinks.length > 0) {
			logger.info(markup`[Cache] Wrote entries due to ${reason}`);
			logger.list(filelinks);
		}
	}

	public addPendingWrite(path: AbsoluteFilePath, op: WriteOperation) {
		if (this.disabled) {
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
