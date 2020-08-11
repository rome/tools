/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticOrigin} from "@internal/diagnostics";
import {TestServerRunnerOptions} from "../../server/testing/types";
import {Bridge} from "@internal/events";
import {FileReference} from "../types/files";
import {
	FocusedTest,
	TestWorkerFileResult,
} from "@internal/core/test-worker/TestWorkerRunner";

export type TestRef = {
	filename: string;
	testName: string;
};

export type TestWorkerPrepareTestOptions = {
	id: number;
	file: FileReference;
	projectDirectory: string;
	code: string;
	cwd: string;
	options: TestServerRunnerOptions;
};

export type TestWorkerPrepareTestResult = {
	focusedTests: Array<FocusedTest>;
};

export type TestWorkerRunTestOptions = {
	id: number;
	onlyFocusedTests: boolean;
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

	public prepareTest = this.createEvent<
		TestWorkerPrepareTestOptions,
		TestWorkerPrepareTestResult
	>({
		name: "prepareTest",
		direction: "server->client",
	});

	public runTest = this.createEvent<
		TestWorkerRunTestOptions,
		TestWorkerFileResult
	>({
		name: "runTest",
		direction: "server->client",
	});

	public testsFound = this.createEvent<Array<TestRef>, void>({
		name: "onTestFounds",
		direction: "server<-client",
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
