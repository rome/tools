/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CHILD_ARGS, VERSION, getBinPath} from "@internal/core";
import workerThreads = require("worker_threads");
import child = require("child_process");
import {Dict} from "@internal/typescript-helpers";

function createEnv(processType: string): Dict<string> {
	return {
		...process.env,
		ROME_PROCESS_VERSION: VERSION,
		ROME_PROCESS_TYPE: processType,
	};
}

export function forkProcess(
	processType: string,
	opts: child.ForkOptions = {},
	args: Array<string> = [],
): child.ChildProcess {
	return child.fork(
		getBinPath().join(),
		args,
		{
			stdio: "inherit",
			execArgv: CHILD_ARGS,
			...opts,
			env: createEnv(processType),
		},
	);
}

export function forkThread(
	processType: string,
	opts: workerThreads.WorkerOptions = {},
): workerThreads.Worker {
	return new workerThreads.Worker(
		`require(${JSON.stringify(getBinPath().join())});`,
		{
			...opts,
			eval: true,
			env: createEnv(processType),
		},
	);
}
