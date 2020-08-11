/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {deriveDiagnosticFromError} from "@internal/diagnostics";
import {TestWorkerBridge} from "@internal/core";
import {createBridgeFromWorkerThreadParentPort} from "@internal/events";
import TestWorkerRunner from "./TestWorkerRunner";
import inspector = require("inspector");
import setupGlobalErrorHandlers from "@internal/core/common/utils/setupGlobalErrorHandlers";

export type TestWorkerFlags = {
	inspectorPort: number;
};

export default class TestWorker {
	constructor() {}

	public async init(flags: TestWorkerFlags) {
		const runners: Map<number, TestWorkerRunner> = new Map();

		const bridge = await createBridgeFromWorkerThreadParentPort(
			TestWorkerBridge,
			{
				type: "server",
			},
		);

		setupGlobalErrorHandlers((err) => {
			bridge.testDiagnostic.send({
				origin: undefined,
				diagnostic: deriveDiagnosticFromError(
					err,
					{
						description: {
							category: "tests/unhandledRejection",
						},
					},
				),
			});
		});

		bridge.inspectorDetails.subscribe(() => {
			return {
				inspectorUrl: inspector.url(),
			};
		});

		bridge.prepareTest.subscribe((opts) => {
			const runner = new TestWorkerRunner(opts, bridge);
			runners.set(opts.id, runner);
			return runner.prepare();
		});

		bridge.runTest.subscribe((opts) => {
			const {id} = opts;
			const runner = runners.get(id);
			if (runner === undefined) {
				throw new Error(`No runner ${id} found`);
			} else {
				return runner.run(opts);
			}
		});

		inspector.open(flags.inspectorPort);

		await bridge.handshake();
	}
}
