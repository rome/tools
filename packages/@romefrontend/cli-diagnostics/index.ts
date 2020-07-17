/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from "@romefrontend/diagnostics";
import {DiagnosticsPrinterOptions} from "./types";
import {Reporter, ReporterStream} from "@romefrontend/cli-reporter";
import DiagnosticsPrinter from "./DiagnosticsPrinter";
import {TerminalFeatures} from "@romefrontend/cli-environment";

export {toLines} from "./utils";
export {
	DEFAULT_PRINTER_FLAGS,
	readDiagnosticsFileLocal,
} from "./DiagnosticsPrinter";
export {DiagnosticsPrinter};

export * from "./constants";

export * from "./types";

export * from "./utils";

export function printDiagnostics(
	{
		diagnostics,
		suppressions,
		printerOptions,
		excludeFooter,
	}: {
		diagnostics: Diagnostics;
		suppressions: DiagnosticSuppressions;
		printerOptions: DiagnosticsPrinterOptions;
		excludeFooter?: boolean;
	},
): DiagnosticsPrinter {
	const printer = new DiagnosticsPrinter(printerOptions);
	printer.processor.addDiagnostics(diagnostics);
	printer.processor.addSuppressions(suppressions);
	printer.print();
	if (!excludeFooter || !printer.hasProblems()) {
		printer.footer();
	}
	return printer;
}

export function printDiagnosticsToString(
	opts: {
		diagnostics: Diagnostics;
		suppressions: DiagnosticSuppressions;
		printerOptions?: DiagnosticsPrinterOptions;
		format?: ReporterStream["format"];
		excludeFooter?: boolean;
		features?: Partial<TerminalFeatures>;
	},
): string {
	const reporter = new Reporter();
	const stream = reporter.attachCaptureStream(opts.format, opts.features);
	printDiagnostics({
		...opts,
		printerOptions: {
			reporter,
			...opts.printerOptions,
		},
	});
	return stream.read();
}
