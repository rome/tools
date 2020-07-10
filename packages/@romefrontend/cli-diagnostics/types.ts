/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@romefrontend/cli-reporter";
import {AbsoluteFilePath} from "@romefrontend/path";
import {DiagnosticsProcessor} from "@romefrontend/diagnostics";

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

export type DiagnosticsFileReader = (
	path: AbsoluteFilePath,
) => undefined | DiagnosticsFileReaderStats;

export type DiagnosticsFileReaderStats = {
	content: string;
	mtime: number;
};

export type DiagnosticsPrinterOptions = {
	processor?: DiagnosticsProcessor;
	reporter: Reporter;
	cwd?: AbsoluteFilePath;
	flags?: DiagnosticsPrinterFlags;
	readFile?: DiagnosticsFileReader;
};
