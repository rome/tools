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

export default async function worker() {
	setProcessTitle("worker");
	const bridge = createBridgeFromWorkerThreadParentPort(
		WorkerBridge,
		{
			type: "server",
		},
	);

	const userConfig = await loadUserConfig();
	const worker = new Worker({
		userConfig,
		bridge,
		dedicated: true,
	});
	await worker.init();
	await bridge.handshake();
}
