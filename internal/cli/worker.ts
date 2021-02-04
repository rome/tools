/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from "./utils/setProcessTitle";
import {Worker, WorkerBridge} from "@internal/core";
import {loadUserConfig} from "@internal/core/common/userConfig";
import workerThreads = require("worker_threads");
import {consumeUnknown} from "@internal/consume";

export default async function worker() {
	setProcessTitle("worker");
	const bridge = WorkerBridge.Client.createFromWorkerThreadParentPort();
	const workerData = consumeUnknown(workerThreads.workerData, "parse");

	const userConfig = await loadUserConfig();
	const worker = new Worker({
		userConfig,
		bridge,
		dedicated: true,
		id: workerData.get("id").asNumber(),
		cacheWriteDisabled: workerData.get("cacheWriteDisabled").asBoolean(),
		cacheReadDisabled: workerData.get("cacheReadDisabled").asBoolean(),
	});
	await worker.init();
	await bridge.handshake();
}
