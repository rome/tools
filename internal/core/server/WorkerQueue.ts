/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WorkerContainer} from "@internal/core";
import Server from "./Server";
import {AbsoluteFilePath} from "@internal/path";
import {FileNotFound} from "@internal/fs";
import {Queue, QueueOptions, promiseAllFrom} from "@internal/async";

export type WorkerQueueOptions<M> = QueueOptions<
	{
		path: AbsoluteFilePath;
		item: M;
	},
	WorkerContainer
>;

export default class WorkerQueue<M>
	extends Queue<
		{
			path: AbsoluteFilePath;
			item: M;
		},
		WorkerContainer
	> {
	constructor(server: Server, opts: WorkerQueueOptions<M>) {
		super(opts);
		this.server = server;
	}

	private server: Server;

	// Prematurely fetch the owners so we don't waterfall worker creation
	public async prepare(paths: Iterable<AbsoluteFilePath>) {
		await promiseAllFrom(
			paths,
			async (path) => {
				return FileNotFound.allowMissing(
					path,
					() => this.server.fileAllocator.getOrAssignOwner(path),
				);
			},
		);
	}

	public async pushPath(path: AbsoluteFilePath, metadata: M) {
		const worker = await this.server.fileAllocator.getOrAssignOwner(path);
		await this.pushQueue(worker, {path, item: metadata});
	}
}
