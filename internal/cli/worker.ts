/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from "./utils/setProcessTitle";
import {Worker, WorkerBridge, WorkerOptions} from "@internal/core";
import {loadUserConfig} from "@internal/core/common/userConfig";
import workerThreads = require("worker_threads");
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {consumeUnknown} from "@internal/consume";
import {BridgeClient} from "@internal/events";

async function deriveWorkerOptions(
	bridge: BridgeClient<typeof WorkerBridge>,
): Promise<WorkerOptions> {
	const workerData = consumeUnknown(
		workerThreads.workerData,
		DIAGNOSTIC_CATEGORIES.parse,
	);

	const userConfig = await loadUserConfig();

	let env: WorkerOptions["env"] = {};
	for (const [key, value] of workerData.get("env").asMap()) {
		env[key] = value.asStringOrVoid();
	}

	return {
		userConfig,
		bridge,
		env,
		dedicated: true,
		type: workerData.get("type").asStringSet(["file-processor", "test-runner", "script-runner"]),
		id: workerData.get("id").asNumber(),
		cacheWriteDisabled: workerData.get("cacheWriteDisabled").asBoolean(),
		cacheReadDisabled: workerData.get("cacheReadDisabled").asBoolean(),
		inspectorPort: workerData.get("inspectorPort").asNumberOrVoid(),
	};
}

export default async function worker() {
	setProcessTitle("worker");
	const {bridge} = WorkerBridge.Client.createFromWorkerThreadParentPort();
	const opts = await deriveWorkerOptions(bridge);

	const worker = new Worker(opts);
	await worker.init();
	await bridge.handshake();
}
