/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@internal/cli-reporter";
import {AbsoluteFilePath, RelativeFilePath} from "@internal/path";
import {DiagnosticsProcessor} from "@internal/diagnostics";
import {FSReadStream} from "@internal/fs";

export type DiagnosticPrinterAuxiliaryFormat = undefined | "github-actions";

export type DiagnosticsPrinterFlags = {
	auxiliaryDiagnosticFormat: DiagnosticPrinterAuxiliaryFormat;
	grep: string;
	fieri: boolean;
	inverseGrep: boolean;
	verboseDiagnostics: boolean | "NO_TRUNCATE";
	maxDiagnostics: number;
	showAllDiagnostics: boolean;
};

export type DiagnosticsFileHandler = {
	readRelative?: (path: RelativeFilePath) => Promise<undefined | string>;
	readAbsolute?: (
		path: AbsoluteFilePath,
	) => Promise<undefined | string | FSReadStream>;
	exists?: (path: AbsoluteFilePath) => Promise<undefined | boolean>;
};

export type DiagnosticsPrinterOptions = {
	processor: DiagnosticsProcessor;
	reporter: Reporter;
	wrapErrors?: boolean;
	cwd?: AbsoluteFilePath;
	flags?: DiagnosticsPrinterFlags;
	fileHandlers?: DiagnosticsFileHandler[];
};
