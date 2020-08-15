/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from "./utils/setProcessTitle";
import {TestWorker} from "@internal/core";
import workerThreads = require("worker_threads");

export default async function testWorker() {
	setProcessTitle("test-worker");

	const {inspectorPort} = workerThreads.workerData;
	if (typeof inspectorPort !== "number") {
		throw new Error(
			`Expected inspectorPort to be a number but got ${inspectorPort}`,
		);
	}

	const worker = new TestWorker();
	await worker.init({
		inspectorPort,
	});
}
