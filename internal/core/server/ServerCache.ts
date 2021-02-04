import {getEnvVar} from "@internal/cli-environment";
import {VERSION} from "@internal/core";
import {
	createDirectory,
	exists,
	readFileText,
	removeDirectory,
} from "@internal/fs";
import {AbsoluteFilePath} from "@internal/path";

import {Server} from "..";
import Cache from "../common/Cache";
import {markup} from "@internal/markup";

// We populate a file containing a single string that we can use to avoid loading in the cache directory
const BREAKER_BASENAME = "breaker";

// This is defined in core/constants.ts and will include a dev suffix if necessary
const EXPECTED_BREAKER_VALUE = VERSION;

export default class ServerCache extends Cache {
	constructor(server: Server) {
		let disabled = false;
		if (getEnvVar("ROME_DEV").type === "ENABLED") {
			disabled = true;
		}
		if (getEnvVar("ROME_CACHE").type === "DISABLED") {
			disabled = true;
		}
		if (server.options.forceCacheEnabled) {
			disabled = false;
		}

		super(
			"server",
			{
				userConfig: server.userConfig,
				parentLogger: server.logger,
				readDisabled: disabled,
				writeDisabled: disabled,
				fatalErrorHandler: server.fatalErrorHandler,
			},
		);
		this.server = server;
		this.breakerPath = this.directoryPath.append(BREAKER_BASENAME);
	}

	private server: Server;
	private breakerPath: AbsoluteFilePath;

	public async init() {
		const {memoryFs} = this.server;
		await createDirectory(this.directoryPath);
		await memoryFs.watch(this.directoryPath);

		await this.initBreaker();

		this.server.endEvent.subscribe(async () => {
			await this.teardown();
		});
	}

	public async initBreaker(): Promise<void> {
		const {breakerPath, logger} = this;

		if (this.readDisabled) {
			logger.warn(markup`Read disabled, skipping breaker verification`);
		}

		if (await exists(breakerPath)) {
			const content = await readFileText(breakerPath);
			if (content === EXPECTED_BREAKER_VALUE) {
				logger.success(markup`Breaker is correct`);
				return;
			} else {
				logger.warn(
					markup`Breaker does not match. Expected ${EXPECTED_BREAKER_VALUE} but got ${content}`,
				);
			}
		} else {
			logger.warn(markup`Breaker does not exist`);
		}

		this.readDisabled = true;
		this.addPendingWrite(
			this.breakerPath,
			{
				type: "update",
				value: EXPECTED_BREAKER_VALUE,
			},
		);
	}

	public async clear() {
		this.pendingWrites.clear();
		await this.server.fileAllocator.evictAll();
		await removeDirectory(this.directoryPath);
	}
}
