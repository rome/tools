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
import stringDiff, {Diff, DiffTypes} from "@internal/string-diff";
import {ZeroIndexed} from "@internal/numbers";
import {Position} from "@internal/parser-core";
import {
	Diagnostic,
	DiagnosticAdviceAction,
	DiagnosticLocation,
	formatCategoryDescription,
} from "@internal/diagnostics";
import {Server, WorkerBufferPatch} from "@internal/core";

export function convertPositionToLSP(pos: undefined | Position): LSPPosition {
	if (pos === undefined) {
		return {
			line: 0,
			character: 0,
		};
	} else {
		return {
			line: pos.line.toZeroIndexed().valueOf(),
			character: pos.column.valueOf(),
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
	diagnostics: Diagnostic[],
	server: Server,
): LSPDiagnostic[] {
	const lspDiagnostics: LSPDiagnostic[] = [];

	for (const {description, location} of diagnostics) {
		// Infer relatedInformation from log messages followed by frames
		let relatedInformation: LSPDiagnosticRelatedInformation[] = [];
		const {advice} = description;
		for (let i = 0; i < advice.length; i++) {
			const item = advice[i];
			const nextItem = advice[i + 1];
			if (
				item.type === "log" &&
				nextItem !== undefined &&
				nextItem.type === "frame"
			) {
				const abs = server.projectManager.getFilePathFromUIDOrAbsolute(
					nextItem.location.path,
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
			code: formatCategoryDescription(description),
			source: "rome",
			relatedInformation,
		});
	}

	return lspDiagnostics;
}

export function getPathFromTextDocument(consumer: Consumer): AbsoluteFilePath {
	return createAbsoluteFilePath(consumer.get("uri").asString());
}

export function diffTextEdits(original: string, desired: string): LSPTextEdit[] {
	const edits: LSPTextEdit[] = [];

	const diffs: Diff[] = stringDiff(original, desired);

	let currLine: ZeroIndexed = new ZeroIndexed();
	let currChar: ZeroIndexed = new ZeroIndexed();

	function advance(str: string) {
		for (const char of str) {
			if (char === "\n") {
				currLine = currLine.increment();
				currChar = new ZeroIndexed();
			} else {
				currChar = currChar.increment();
			}
		}
	}

	function getPosition(): LSPPosition {
		return {
			line: currLine.valueOf(),
			character: currChar.valueOf(),
		};
	}

	for (const [type, text] of diffs) {
		switch (type) {
			case DiffTypes.INSERT: {
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

			case DiffTypes.DELETE: {
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

			case DiffTypes.EQUAL: {
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
			line: start.get("line").asNumber(),
			character: start.get("character").asNumber(),
		},
		end: {
			line: end.get("line").asNumber(),
			character: end.get("character").asNumber(),
		},
	};
}

function getWorkerBufferPatchFromLSPRange(
	range: Consumer,
): WorkerBufferPatch["range"] {
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
): WorkerBufferPatch[] {
	return contentChanges.asMappedArray((change) => {
		const range = getWorkerBufferPatchFromLSPRange(change.get("range"));

		return {
			text: change.get("text").asString(),
			range,
		};
	});
}

export function getDecisionFromAdviceAction(
	advice: DiagnosticAdviceAction,
): string | undefined {
	const decisions = advice?.commandFlags?.decisions;
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
