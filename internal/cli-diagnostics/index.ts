/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticSuppression,
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
	}: {
		diagnostics: Diagnostic[];
		suppressions: DiagnosticSuppression[];
		printerOptions: DiagnosticsPrinterOptions;
	},
): Promise<DiagnosticsPrinter> {
	const printer = new DiagnosticsPrinter(printerOptions);
	printer.processor.addDiagnostics(diagnostics);
	printer.processor.addSuppressions(suppressions);
	await printer.print({
		showFooter: false,
	});
	return printer;
}

export async function printDiagnosticsToString(
	opts: {
		diagnostics: Diagnostic[];
		suppressions: DiagnosticSuppression[];
		printerOptions?: Partial<DiagnosticsPrinterOptions>;
		format?: ReporterStream["format"];
		features?: Partial<TerminalFeatures>;
	},
): Promise<string> {
	const reporter = new Reporter("DiagnosticsPrinter");
	const stream = reporter.attachCaptureStream(opts.format, opts.features);
	await printDiagnostics({
		...opts,
		printerOptions: {
			reporter,
			processor: new DiagnosticsProcessor(),
			...opts.printerOptions,
		},
	});
	await reporter.resources.release();
	return stream.read();
}
