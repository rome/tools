import {createDirectory, removeDirectory} from "@internal/fs";
import {Server} from "..";
import Cache from "../common/Cache";

export default class ServerCache extends Cache {
	constructor(server: Server) {
		super(
			"server",
			{
				userConfig: server.userConfig,
				logger: server.logger,
				forceEnabled: server.options.forceCacheEnabled,
				fatalErrorHandler: server.fatalErrorHandler,
			},
		);
		this.server = server;
	}

	private server: Server;

	public async init() {
		const {memoryFs} = this.server;
		await createDirectory(this.cachePath);
		await memoryFs.watch(this.cachePath);

		this.server.endEvent.subscribe(async () => {
			await this.teardown();
		});
	}

	public async clear() {
		this.pendingWrites.clear();
		await this.server.fileAllocator.evictAll();
		await removeDirectory(this.cachePath);
	}
}
