/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {deriveDiagnosticFromError} from "@internal/diagnostics";
import {TestWorkerBridge} from "@internal/core";
import {createBridgeFromWorkerThreadParentPort} from "@internal/events";
import TestWorkerFile from "./TestWorkerFile";
import inspector = require("inspector");
import {AbsoluteFilePathMap} from "@internal/path";
import {serializeAssembled} from "../server/bundler/utils";
import {AssembledBundle} from "../common/types/bundler";
import setupGlobalErrorHandlers from "@internal/core/common/utils/setupGlobalErrorHandlers";

export type TestWorkerFlags = {
	inspectorPort: number;
};

export default class TestWorker {
	constructor() {
		this.bridge = this.buildBridge();
		this.runners = new AbsoluteFilePathMap();
		this.compiled = new AbsoluteFilePathMap();
	}

	private runners: AbsoluteFilePathMap<TestWorkerFile>;
	private bridge: TestWorkerBridge;
	private compiled: AbsoluteFilePathMap<string>;

	public serializeAssembled(assembled: AssembledBundle): string {
		return serializeAssembled(
			assembled,
			(path) => {
				return this.compiled.get(path);
			},
		);
	}

	public async init(flags: TestWorkerFlags) {
		inspector.open(flags.inspectorPort);

		await this.bridge.handshake();
	}

	private buildBridge(): TestWorkerBridge {
		const bridge = createBridgeFromWorkerThreadParentPort(
			TestWorkerBridge,
			{
				type: "server",
			},
		);

		setupGlobalErrorHandlers((err) => {
			bridge.testDiagnostic.send({
				testPath: undefined,
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

		bridge.prepareTest.subscribe(async (opts) => {
			const runner = new TestWorkerFile(this, this.bridge, opts);
			this.runners.set(opts.path, runner);
			return await runner.prepare();
		});

		bridge.teardownTest.subscribe(async (path) => {
			const runner = this.runners.assert(path);
			return await runner.teardown();
		});

		bridge.runTest.subscribe(async (opts) => {
			const {path} = opts;
			const runner = this.runners.assert(path);
			return await runner.run(opts);
		});

		bridge.receiveCompiled.subscribe((map) => {
			for (const [path, content] of map) {
				this.compiled.set(path, content);
			}
		});

		return bridge;
	}
}
