/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@internal/cli-reporter";
import {AbsoluteFilePath, Path} from "@internal/path";
import {DiagnosticsProcessor} from "@internal/diagnostics";
import {FSReadStream} from "@internal/fs";

export type DiagnosticsPrinterAuxiliaryFormat = undefined | "github-actions";

export type DiagnosticsPrinterFlags = {
	auxiliaryDiagnosticFormat: DiagnosticsPrinterAuxiliaryFormat;
	fieri: boolean;
	truncateDiagnostics: boolean;
	verboseDiagnostics: boolean;
};

export type DiagnosticsFileHandler = {
	read?: (path: Path) => Promise<undefined | string | FSReadStream>;
	exists?: (path: Path) => Promise<undefined | boolean>;
};

export type DiagnosticsPrinterOptions = {
	processor: DiagnosticsProcessor;
	reporter: Reporter;
	streaming?: boolean;
	wrapErrors?: boolean;
	cwd?: AbsoluteFilePath;
	flags?: Partial<DiagnosticsPrinterFlags>;
	fileHandlers?: DiagnosticsFileHandler[];
};
