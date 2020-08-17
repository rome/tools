/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticOrigin} from "@internal/diagnostics";
import {TestServerRunnerOptions} from "../../server/testing/types";
import {Bridge} from "@internal/events";
import {
	FocusedTest,
	TestWorkerFileResult,
} from "@internal/core/test-worker/TestWorkerFile";
import {AssembledBundle} from "@internal/core";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";

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
	foundTests: Array<string>;
	focusedTests: Array<FocusedTest>;
};

export type TestWorkerRunTestOptions = {
	path: AbsoluteFilePath;
	testNames: Array<string>;
};

export default class TestWorkerBridge extends Bridge {
	public inspectorDetails = this.createEvent<
		void,
		{
			inspectorUrl: undefined | string;
		}
	>({
		name: "inspectorDetails",
		direction: "server->client",
	});

	public receiveCompiled = this.createEvent<AbsoluteFilePathMap<string>, void>({
		name: "receiveCompiled",
		direction: "server->client",
	});

	public prepareTest = this.createEvent<
		TestWorkerPrepareTestOptions,
		TestWorkerPrepareTestResult
	>({
		name: "prepareTest",
		direction: "server->client",
	});

	public runTest = this.createEvent<TestWorkerRunTestOptions, void>({
		name: "runTest",
		direction: "server->client",
	});

	public teardownTest = this.createEvent<AbsoluteFilePath, TestWorkerFileResult>({
		name: "teardownTest",
		direction: "server->client",
	});

	public testStart = this.createEvent<
		{
			ref: TestRef;
			timeout: undefined | number;
		},
		void
	>({
		name: "onTestStart",
		direction: "server<-client",
	});

	public testDiagnostic = this.createEvent<
		{
			testPath: undefined | AbsoluteFilePath;
			diagnostic: Diagnostic;
			origin: undefined | DiagnosticOrigin;
		},
		void
	>({
		name: "testDiagnostic",
		direction: "server<-client",
	});

	public testFinish = this.createEvent<
		{
			success: boolean;
			ref: TestRef;
		},
		void
	>({
		name: "onTestSuccess",
		direction: "server<-client",
	});
}
