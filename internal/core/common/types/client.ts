/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DEFAULT_PRINTER_FLAGS,
	DiagnosticsPrinterFlags,
} from "@internal/cli-diagnostics";
import {Platform} from "./platform";
import {AbsoluteFilePath, CWD_PATH} from "@internal/path";
import {ReporterStream} from "@internal/cli-reporter";
import {TerminalFeatures} from "@internal/cli-environment";
import {
	ServerQueryResponse,
	ServerQueryResponseBase,
} from "../bridges/ServerBridge";
import {
	DEFAULT_PROCESSOR_FILTER_FLAGS,
	DiagnosticsProcessorFilterOptions,
} from "@internal/diagnostics";

export const DEFAULT_CLIENT_FLAGS: ClientFlags = {
	clientName: "unknown",
	cwd: CWD_PATH,
	realCwd: CWD_PATH,
	silent: false,
};

export const DEFAULT_CLIENT_REQUEST_FLAGS: ClientRequestFlags = {
	...DEFAULT_PRINTER_FLAGS,
	...DEFAULT_PROCESSOR_FILTER_FLAGS,
	programmatic: true,
	unsafeWrites: false,
	collectMarkers: false,
	timing: false,
	benchmark: false,
	benchmarkIterations: 10,
	watch: false,
	review: false,
	resolverPlatform: undefined,
	resolverScale: undefined,
	resolverMocks: false,
};

export type ClientRequestFlags = DiagnosticsPrinterFlags &
	DiagnosticsProcessorFilterOptions & {
		watch: boolean;
		review: boolean;
		programmatic: boolean;
		unsafeWrites: boolean;

		// Debugging
		timing: boolean;
		collectMarkers: boolean;
		benchmark: boolean;
		benchmarkIterations: number;

		// Bundler flags
		resolverPlatform: undefined | Platform;
		resolverScale: undefined | number;
		resolverMocks: boolean;
	};

export type ClientTerminalFeatures = Partial<TerminalFeatures> & {
	redirectError?: boolean;
	format?: ReporterStream["format"];
};

export type ClientFlags = {
	clientName: string;
	cwd: AbsoluteFilePath;
	realCwd: AbsoluteFilePath;
	silent: boolean;
};

export type ClientLogsLevel = "all" | "error";

export type ClientQueryErrorResponse = ServerQueryResponseBase & {
	type: "CLIENT_ERROR";
	message: string;
};

export type ClientQueryResponse = ServerQueryResponse | ClientQueryErrorResponse;
