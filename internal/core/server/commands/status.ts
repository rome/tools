/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {WorkerStatus} from "../../common/bridges/WorkerBridge";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {markup} from "@internal/markup";

type StatusResult = {
	server: {
		heapTotal: number;
		pid: number;
		uptime: number;
	};
	workers: Array<StatusWorkerResult>;
	projects: Array<{
		id: number;
	}>;
};

type StatusWorkerResult = {
	astCacheSize: number;
	heapTotal: number;
	pid: number;
	uptime: number;
	ownedBytes: number;
	ownedFileCount: number;
};

export default createServerCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`dump memory and process info of server and workers`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback({server}: ServerRequest): Promise<StatusResult> {
		const workers = await Promise.all(
			server.workerManager.getWorkers().map(async (
				worker,
			): Promise<StatusWorkerResult> => {
				const workerStatus: WorkerStatus = await worker.bridge.status.call();

				return {
					astCacheSize: workerStatus.astCacheSize,
					heapTotal: workerStatus.memoryUsage.heapTotal,
					pid: workerStatus.pid,
					uptime: workerStatus.uptime,
					ownedBytes: worker.byteCount,
					ownedFileCount: worker.fileCount,
				};
			}),
		);

		const {heapTotal} = process.memoryUsage();
		return {
			server: {
				heapTotal,
				pid: process.pid,
				uptime: process.uptime(),
			},
			workers,
			projects: server.projectManager.getProjects().map((project) => {
				return {
					id: project.id,
				};
			}),
		};
	},
});
