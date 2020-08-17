/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from "./utils/setProcessTitle";
import {createBridgeFromWorkerThreadParentPort} from "@internal/events";
import {Worker, WorkerBridge} from "@internal/core";
import {loadUserConfig} from "@internal/core/common/userConfig";
import workerThreads = require("worker_threads");

export default async function worker() {
	setProcessTitle("worker");
	const bridge = createBridgeFromWorkerThreadParentPort(
		WorkerBridge,
		{
			type: "server",
		},
	);

	const {id} = workerThreads.workerData;
	if (typeof id !== "number") {
		throw new Error(`Expected id to be a number but got ${id}`);
	}

	const userConfig = await loadUserConfig();
	const worker = new Worker({
		id,
		userConfig,
		bridge,
		dedicated: true,
	});
	await worker.init();
	await bridge.handshake();
}
