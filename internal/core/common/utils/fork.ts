/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CHILD_ARGS, VERSION, getBinPath} from "@internal/core";
import workerThreads = require("worker_threads");
import child = require("child_process");

function createEnv(
	processType: string,
	env?: NodeJS.ProcessEnv,
): NodeJS.ProcessEnv {
	return {
		...process.env,
		...env,
		ROME_PROCESS_VERSION: VERSION,
		ROME_PROCESS_TYPE: processType,
	};
}

export function forkProcess(
	processType: string,
	opts: child.ForkOptions = {},
	args: string[] = [],
): child.ChildProcess {
	return child.fork(
		getBinPath().join(),
		args,
		{
			stdio: "inherit",
			execArgv: CHILD_ARGS,
			...opts,
			env: createEnv(processType, opts.env),
		},
	);
}

export function forkThread(
	processType: string,
	opts: Omit<workerThreads.WorkerOptions, "env"> & {
		env?: NodeJS.ProcessEnv;
	} = {},
): workerThreads.Worker {
	return new workerThreads.Worker(
		`require(${JSON.stringify(getBinPath().join())});`,
		{
			...opts,
			execArgv: CHILD_ARGS,
			eval: true,
			env: createEnv(processType, opts.env),
		},
	);
}
