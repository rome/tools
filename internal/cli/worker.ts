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

export default async function worker() {
	setProcessTitle("worker");
	const bridge = WorkerBridge.Client.createFromWorkerThreadParentPort();

	const {id, cacheDisabled} = workerThreads.workerData;
	if (typeof id !== "number") {
		throw new Error(`Expected id to be a number but got ${id}`);
	}
	if (typeof cacheDisabled !== "boolean") {
		throw new Error(
			`Expected cacheDisabled to be a boolean but got ${cacheDisabled}`,
		);
	}

	const userConfig = await loadUserConfig();
	const worker = new Worker({
		id,
		userConfig,
		bridge,
		dedicated: true,
		cacheDisabled,
	});
	await worker.init();
	await bridge.handshake();
}
