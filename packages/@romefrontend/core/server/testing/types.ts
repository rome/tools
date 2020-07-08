/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics} from "@romefrontend/diagnostics";
import {SourceMapGenerator} from "@romefrontend/codec-source-map";
import {ServerRequest, TestWorkerBridge} from "@romefrontend/core";
import {CoverageFile, InspectorClient} from "@romefrontend/v8";
import child = require("child_process");
import {FileReference} from "@romefrontend/core/common/types/files";

export type TestSource = {
	code: string;
	sourceMap: SourceMapGenerator;
	ref: FileReference;
};

export type TestSources = Map<string, TestSource>;

export type TestServerRunnerConstructorOptions = {
	sources: TestSources;
	request: ServerRequest;
	addDiagnostics: Diagnostics;
	options: TestServerRunnerOptions;
};

export type TestServerRunnerOptions = {
	filter: undefined | string;
	focusAllowed: boolean;
	coverage: boolean;
	showAllCoverage: boolean;
	updateSnapshots: boolean;
	freezeSnapshots: boolean;
	verboseDiagnostics: boolean;
	syncTests: boolean;
};

export type CoverageFolder = {
	name: undefined | string;
	folders: Map<string, CoverageFolder>;
	files: Map<string, CoverageFile>;
};

export type TestWorkerContainer = {
	bridge: TestWorkerBridge;
	process: child.ChildProcess;
	inspector: undefined | InspectorClient;
};

export type TestWorkerContainers = Array<TestWorkerContainer>;
