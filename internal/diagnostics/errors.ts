/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics, DiagnosticsProcessor} from "@internal/diagnostics";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {Diagnostic, DiagnosticSuppressions} from "./types";
import {Reporter} from "@internal/cli-reporter";
import {readMarkup} from "@internal/markup";
import {
	DeriveErrorDiagnosticOptions,
	deriveDiagnosticFromError,
} from "./derive";
import {
	NodeSystemError,
	convertPossibleNodeErrorToDiagnostic,
} from "@internal/node";

// If printDiagnosticsToString throws a DiagnosticsError then we'll be trapped in a loop forever
// since we'll continuously be trying to serialize diagnostics
let insideDiagnosticsErrorSerial = false;

export class DiagnosticsError extends Error implements NodeSystemError {
	constructor(
		message: undefined | string,
		diagnostics: Diagnostics,
		suppressions: DiagnosticSuppressions = [],
	) {
		if (diagnostics.length === 0) {
			throw new Error("No diagnostics");
		}

		super();
		this._memoMessage = undefined;
		this._message = message;
		this.diagnostics = diagnostics;
		this.suppressions = suppressions;
		this.name = "DiagnosticsError";
	}

	private _memoMessage: string | undefined;
	private _message: undefined | string;

	// Lazily instantiate this. If we ever catchDiagnostics we wont even care about the `message`
	// so this avoids having to print it to a string
	public get message(): string {
		if (this._memoMessage !== undefined) {
			return this._memoMessage;
		}

		if (insideDiagnosticsErrorSerial) {
			return [
				"Possible DiagnosticsError message serialization infinite loop",
				"Diagnostic messages:",
				this.diagnostics.map((diag) =>
					`- ${readMarkup(diag.description.message)}`
				),
			].join("\n");
		}

		let message = this._message === undefined ? "" : this._message + "\n";
		insideDiagnosticsErrorSerial = true;

		const reporter = new Reporter();
		const stream = reporter.attachCaptureStream("none", {columns: undefined});
		const printer = new DiagnosticsPrinter({
			reporter,
			processor: new DiagnosticsProcessor(),
			wrapErrors: true,
		});
		for (const diag of this.diagnostics) {
			printer.printDiagnostic(diag);
		}
		message += stream.read();
		insideDiagnosticsErrorSerial = false;

		this._memoMessage = message;
		return message;
	}

	public diagnostics: Diagnostics;
	public suppressions: DiagnosticSuppressions;
}

export function createSingleDiagnosticError(
	diag: Diagnostic,
	suppressions?: DiagnosticSuppressions,
): DiagnosticsError {
	return new DiagnosticsError(
		readMarkup(diag.description.message),
		[diag],
		suppressions,
	);
}

export function getDiagnosticsFromError(err: Error): undefined | Diagnostics {
	if (err instanceof DiagnosticsError) {
		const processor = new DiagnosticsProcessor({});
		processor.addSuppressions(err.suppressions);
		processor.addDiagnostics(err.diagnostics);
		return processor.getDiagnostics();
	}

	return undefined;
}

export function getOrDeriveDiagnosticsFromError(
	err: Error,
	opts: DeriveErrorDiagnosticOptions,
): Diagnostics {
	err = convertPossibleNodeErrorToDiagnostic(err);
	const diagnostics = getDiagnosticsFromError(err);
	if (diagnostics === undefined) {
		return [deriveDiagnosticFromError(err, opts)];
	} else {
		return diagnostics;
	}
}
