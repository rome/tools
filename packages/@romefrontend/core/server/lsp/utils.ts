import {markupToPlainTextString} from "@romefrontend/string-markup";
import {AbsoluteFilePath, createAbsoluteFilePath} from "@romefrontend/path";
import {Consumer} from "@romefrontend/consume";
import {
	LSPDiagnostic,
	LSPDiagnosticRelatedInformation,
	LSPPosition,
	LSPRange,
	LSPTextEdit,
} from "./types";
import stringDiff, {Diffs, diffConstants} from "@romefrontend/string-diff";
import {Number0, ob1Coerce1To0, ob1Inc, ob1Number0} from "@romefrontend/ob1";
import {Position} from "@romefrontend/parser-core";
import {DiagnosticLocation, Diagnostics} from "@romefrontend/diagnostics";
import {Server} from "@romefrontend/core";
import {WorkerBufferPatch} from "@romefrontend/core/common/bridges/WorkerBridge";

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
						message: markupToPlainTextString(item.text),
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
			message: markupToPlainTextString(description.message.value),
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

export function getWorkerBufferPatches(
	contentChanges: Consumer,
): Array<WorkerBufferPatch> {
	return contentChanges.asArray().map((change) => {
		const start = change.get("range").get("start");
		const end = change.get("range").get("end");

		return {
			text: change.get("text").asString(),
			range: {
				start: {
					line: start.get("line").asZeroIndexedNumber(),
					character: start.get("character").asZeroIndexedNumber(),
				},
				end: {
					line: end.get("line").asZeroIndexedNumber(),
					character: end.get("character").asZeroIndexedNumber(),
				},
			},
		};
	});
}
