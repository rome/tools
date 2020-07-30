/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsProcessor,
} from "@internal/diagnostics";
import {DiagnosticsPrinterOptions} from "./types";
import {Reporter, ReporterStream} from "@internal/cli-reporter";
import DiagnosticsPrinter from "./DiagnosticsPrinter";
import {TerminalFeatures} from "@internal/cli-environment";

export {toLines} from "./utils";
export {DEFAULT_PRINTER_FLAGS} from "./DiagnosticsPrinter";
export {DiagnosticsPrinter};

export * from "./constants";

export * from "./types";

export * from "./utils";

export async function printDiagnostics(
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
): Promise<DiagnosticsPrinter> {
	const printer = new DiagnosticsPrinter(printerOptions);
	printer.processor.addDiagnostics(diagnostics);
	printer.processor.addSuppressions(suppressions);
	await printer.print();
	if (!excludeFooter || !printer.hasProblems()) {
		await printer.footer();
	}
	return printer;
}

export async function printDiagnosticsToString(
	opts: {
		diagnostics: Diagnostics;
		suppressions: DiagnosticSuppressions;
		printerOptions?: DiagnosticsPrinterOptions;
		format?: ReporterStream["format"];
		excludeFooter?: boolean;
		features?: Partial<TerminalFeatures>;
	},
): Promise<string> {
	const reporter = new Reporter();
	const stream = reporter.attachCaptureStream(opts.format, opts.features);
	await printDiagnostics({
		...opts,
		printerOptions: {
			reporter,
			processor: new DiagnosticsProcessor(),
			...opts.printerOptions,
		},
	});
	return stream.read();
}
