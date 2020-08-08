import {markupToJoinedPlainText} from "@internal/cli-layout";
import {AbsoluteFilePath, createAbsoluteFilePath} from "@internal/path";
import {Consumer} from "@internal/consume";
import {
	LSPDiagnostic,
	LSPDiagnosticRelatedInformation,
	LSPPosition,
	LSPRange,
	LSPTextEdit,
} from "./types";
import stringDiff, {Diffs, diffConstants} from "@internal/string-diff";
import {Number0, ob1Coerce1To0, ob1Inc, ob1Number0} from "@internal/ob1";
import {Position} from "@internal/parser-core";
import {
	DiagnosticAdviceAction,
	DiagnosticLocation,
	Diagnostics,
} from "@internal/diagnostics";
import {Server} from "@internal/core";
import {WorkerBufferPatch} from "@internal/core/common/bridges/WorkerBridge";

export function convertPositionToLSP(pos: undefined | Position): LSPPosition {
	if (pos === undefined) {
		return {
			line: ob1Number0,
			character: ob1Number0,
		};
	} else {
		return {
			line: ob1Coerce1To0(pos.line),
			character: pos.column,
		};
	}
}

export function convertDiagnosticLocationToLSPRange(
	location: DiagnosticLocation,
): LSPRange {
	return {
		start: convertPositionToLSP(location.start),
		end: convertPositionToLSP(location.end),
	};
}

export function convertDiagnosticsToLSP(
	diagnostics: Diagnostics,
	server: Server,
): Array<LSPDiagnostic> {
	const lspDiagnostics: Array<LSPDiagnostic> = [];

	for (const {description, location} of diagnostics) {
		// Infer relatedInformation from log messages followed by frames
		let relatedInformation: Array<LSPDiagnosticRelatedInformation> = [];
		const {advice} = description;
		for (let i = 0; i < advice.length; i++) {
			const item = advice[i];
			const nextItem = advice[i + 1];
			if (
				item.type === "log" &&
				nextItem !== undefined &&
				nextItem.type === "frame"
			) {
				const abs = server.projectManager.getFilePathFromUidOrAbsolute(
					nextItem.location.filename,
				);
				if (abs !== undefined) {
					relatedInformation.push({
						message: markupToJoinedPlainText(item.text),
						location: {
							uri: `file://${abs.join()}`,
							range: convertDiagnosticLocationToLSPRange(nextItem.location),
						},
					});
				}
			}
		}

		lspDiagnostics.push({
			severity: 1,
			range: convertDiagnosticLocationToLSPRange(location),
			message: markupToJoinedPlainText(description.message),
			code: description.category,
			source: "rome",
			relatedInformation,
		});
	}

	return lspDiagnostics;
}

export function getPathFromTextDocument(consumer: Consumer): AbsoluteFilePath {
	return createAbsoluteFilePath(consumer.get("uri").asString());
}

export function diffTextEdits(
	original: string,
	desired: string,
): Array<LSPTextEdit> {
	const edits: Array<LSPTextEdit> = [];

	const diffs: Diffs = stringDiff(original, desired);

	let currLine: Number0 = ob1Number0;
	let currChar: Number0 = ob1Number0;

	function advance(str: string) {
		for (const char of str) {
			if (char === "\n") {
				currLine = ob1Inc(currLine);
				currChar = ob1Number0;
			} else {
				currChar = ob1Inc(currChar);
			}
		}
	}

	function getPosition(): LSPPosition {
		return {
			line: currLine,
			character: currChar,
		};
	}

	for (const [type, text] of diffs) {
		switch (type) {
			case diffConstants.ADD: {
				const pos = getPosition();
				edits.push({
					range: {
						start: pos,
						end: pos,
					},
					newText: text,
				});
				break;
			}

			case diffConstants.DELETE: {
				const start: LSPPosition = getPosition();
				advance(text);
				const end: LSPPosition = getPosition();
				edits.push({
					range: {
						start,
						end,
					},
					newText: "",
				});
				break;
			}

			case diffConstants.EQUAL: {
				advance(text);
				break;
			}
		}
	}

	return edits;
}

export function getLSPRange(range: Consumer): LSPRange {
	const start = range.get("start");
	const end = range.get("end");

	return {
		start: {
			line: start.get("line").asZeroIndexedNumber(),
			character: start.get("character").asZeroIndexedNumber(),
		},
		end: {
			line: end.get("line").asZeroIndexedNumber(),
			character: end.get("character").asZeroIndexedNumber(),
		},
	};
}

export function getWorkerBufferPatches(
	contentChanges: Consumer,
): Array<WorkerBufferPatch> {
	return contentChanges.asMappedArray((change) => {
		const range = getLSPRange(change.get("range"));

		return {
			text: change.get("text").asString(),
			range,
		};
	});
}

export function getDecisionFromAdviceAction(
	advice: DiagnosticAdviceAction,
): string | undefined {
	const decisions = (advice?.commandFlags)?.decisions;
	if (Array.isArray(decisions)) {
		return decisions[0];
	}
	return;
}

export function doRangesOverlap(a: LSPRange, b: LSPRange) {
	if (a.start.line > b.end.line || b.start.line > a.end.line) {
		return false;
	}
	if (a.start.line === b.end.line && a.start.character > b.end.character) {
		return false;
	}
	if (b.start.line === a.end.line && b.start.character > a.end.character) {
		return false;
	}
	return true;
}
