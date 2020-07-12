/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics, DiagnosticsProcessor} from "@romefrontend/diagnostics";
import {printDiagnosticsToString} from "@romefrontend/cli-diagnostics";
import {Diagnostic, DiagnosticSuppressions} from "./types";

// If printDiagnosticsToString throws a DiagnosticsError then we'll be trapped in a loop forever
// since we'll continuously be trying to serialize diagnostics
let insideDiagnosticsErrorSerial = false;

export class DiagnosticsError extends Error {
	constructor(
		message: string,
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

	_memoMessage: string | undefined;
	_message: string;

	// Lazily instantiate this. If we ever catchDiagnostics we wont even care about the `message`
	// so this avoids having to print it to a string
	get message(): string {
		if (this._memoMessage !== undefined) {
			return this._memoMessage;
		}

		if (insideDiagnosticsErrorSerial) {
			return [
				"Possible DiagnosticsError message serialization infinite loop",
				"Diagnostic messages:",
				this.diagnostics.map((diag) => `- ${diag.description.message}`),
			].join("\n");
		}

		let message = this._message;
		message += "\n";

		insideDiagnosticsErrorSerial = true;
		message += printDiagnosticsToString({
			diagnostics: this.diagnostics,
			suppressions: this.suppressions,
		});
		insideDiagnosticsErrorSerial = false;

		this._memoMessage = message;
		return message;
	}

	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
}

export function createSingleDiagnosticError(
	diag: Diagnostic,
	suppressions?: DiagnosticSuppressions,
): DiagnosticsError {
	return new DiagnosticsError(
		diag.description.message.value,
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
