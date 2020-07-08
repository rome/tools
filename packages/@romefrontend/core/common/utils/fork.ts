/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BIN, CHILD_ARGS, VERSION} from "@romefrontend/core";
import child = require("child_process");

export default function fork(
	processType: string,
	opts: child.ForkOptions = {},
	args: Array<string> = [],
): child.ChildProcess {
	return child.fork(
		BIN.join(),
		args,
		{
			stdio: "inherit",
			execArgv: CHILD_ARGS,
			...opts,
			env: {
				...process.env,
				ROME_PROCESS_VERSION: VERSION,
				ROME_PROCESS_TYPE: processType,
			},
		},
	);
}
