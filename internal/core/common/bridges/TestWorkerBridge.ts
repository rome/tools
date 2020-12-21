/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticOrigin} from "@internal/diagnostics";
import {TestServerRunnerOptions} from "../../server/testing/types";
import {createBridge} from "@internal/events";
import {
	FocusedTest,
	TestWorkerFileResult,
} from "@internal/core/test-worker/TestWorkerFile";
import {AssembledBundle} from "@internal/core";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import {createBridgeEventDeclaration} from "@internal/events/createBridge";

export type TestRef = {
	path: AbsoluteFilePath;
	testName: string;
};

export type TestWorkerPrepareTestOptions = {
	partial: boolean;
	path: AbsoluteFilePath;
	projectDirectory: string;
	assembled: AssembledBundle;
	cwd: string;
	globalOptions: TestServerRunnerOptions;
	logFound: boolean;
};

export type TestWorkerPrepareTestResult = {
	foundTests: string[];
	focusedTests: FocusedTest[];
};

export type TestWorkerRunTestOptions = {
	path: AbsoluteFilePath;
	testNames: string[];
};

export default createBridge({
	debugName: "test worker",

	shared: {},

	client: {
		inspectorDetails: createBridgeEventDeclaration<
			void,
			{
				inspectorUrl: undefined | string;
			}
		>(),
		receiveCompiled: createBridgeEventDeclaration<
			AbsoluteFilePathMap<string>,
			void
		>(),
		prepareTest: createBridgeEventDeclaration<
			TestWorkerPrepareTestOptions,
			TestWorkerPrepareTestResult
		>(),
		runTest: createBridgeEventDeclaration<TestWorkerRunTestOptions, void>(),
		teardownTest: createBridgeEventDeclaration<
			AbsoluteFilePath,
			TestWorkerFileResult
		>(),
	},

	server: {
		testStart: createBridgeEventDeclaration<
			{
				ref: TestRef;
				timeout: undefined | number;
			},
			void
		>(),
		testDiagnostic: createBridgeEventDeclaration<
			{
				testPath: undefined | AbsoluteFilePath;
				diagnostic: Diagnostic;
				origin: undefined | DiagnosticOrigin;
			},
			void
		>(),
		testFinish: createBridgeEventDeclaration<
			{
				success: boolean;
				ref: TestRef;
			},
			void
		>(),
	},
});
