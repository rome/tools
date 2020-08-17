/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WorkerContainer} from "./WorkerManager";
import Server from "./Server";
import {AbsoluteFilePath} from "@internal/path";
import {FileNotFound} from "@internal/fs/FileNotFound";
import {Queue, QueueOptions} from "@internal/async";

export default class WorkerQueue<M>
	extends Queue<
		{
			path: AbsoluteFilePath;
			item: M;
		},
		WorkerContainer
	> {
	constructor(
		server: Server,
		opts: QueueOptions<
			{
				path: AbsoluteFilePath;
				item: M;
			},
			WorkerContainer
		>,
	) {
		super(opts);
		this.server = server;
	}

	private server: Server;

	// Prematurely fetch the owners so we don't waterfall worker creation
	public async prepare(paths: Iterable<AbsoluteFilePath>) {
		await Promise.all(
			Array.from(
				paths,
				async (path) => {
					return FileNotFound.allowMissing(
						path,
						() => this.server.fileAllocator.getOrAssignOwner(path),
					);
				},
			),
		);
	}

	public async pushPath(path: AbsoluteFilePath, metadata: M) {
		const worker = await this.server.fileAllocator.getOrAssignOwner(path);
		await this.pushQueue(worker, {path, item: metadata});
	}
}
