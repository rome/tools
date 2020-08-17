/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {CoverageFile} from "@internal/v8";
import {DiagnosticsPrinterFlags} from "@internal/cli-diagnostics";
import {AbsoluteFilePathSet} from "@internal/path";

export type TestServerRunnerConstructorOptions = {
	paths: AbsoluteFilePathSet;
	request: ServerRequest;
	options: TestServerRunnerOptions;
};

export type TestServerRunnerOptions = {
	filter: undefined | string;
	focusAllowed: boolean;
	coverage: boolean;
	showAllCoverage: boolean;
	updateSnapshots: boolean;
	freezeSnapshots: boolean;
	verboseDiagnostics: DiagnosticsPrinterFlags["verboseDiagnostics"];
	syncTests: boolean;
	sourceMaps: boolean;
};

export type CoverageDirectory = {
	name: undefined | string;
	directories: Map<string, CoverageDirectory>;
	files: Map<string, CoverageFile>;
};
