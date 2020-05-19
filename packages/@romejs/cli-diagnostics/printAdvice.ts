/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@romejs/cli-reporter";
import {
	Diagnostic,
	DiagnosticAdviceAction,
	DiagnosticAdviceCode,
	DiagnosticAdviceCommand,
	DiagnosticAdviceDiff,
	DiagnosticAdviceFrame,
	DiagnosticAdviceInspect,
	DiagnosticAdviceItem,
	DiagnosticAdviceList,
	DiagnosticAdviceLog,
	DiagnosticAdviceStacktrace,
	diagnosticLocationToMarkupFilelink,
} from "@romejs/diagnostics";
import {Position} from "@romejs/parser-core";
import {ToLines, showInvisibles, toLines} from "./utils";
import buildPatchCodeFrame from "./buildPatchCodeFrame";
import buildMessageCodeFrame from "./buildMessageCodeFrame";
import {escapeMarkup, markupTag} from "@romejs/string-markup";
import {DiagnosticsPrinterFlags} from "./types";
import {ob1Number0Neg1} from "@romejs/ob1";
import DiagnosticsPrinter, {DiagnosticsPrinterFileSources} from "./DiagnosticsPrinter";
import {AbsoluteFilePathSet} from "@romejs/path";
import {RAW_CODE_MAX_LENGTH} from "./constants";
import {Diffs, diffConstants} from "@romejs/string-diff";
import {removeCarriageReturn} from "@romejs/string-utils";
import {serializeCLIFlags} from "@romejs/cli-flags";

type AdvicePrintOptions = {
	printer: DiagnosticsPrinter;
	flags: DiagnosticsPrinterFlags;
	missingFileSources: AbsoluteFilePathSet;
	fileSources: DiagnosticsPrinterFileSources;
	reporter: Reporter;
	diagnostic: Diagnostic;
};

type PrintAdviceResult = {
	printed: boolean;
	truncated: boolean;
};

const DID_PRINT: PrintAdviceResult = {
	printed: true,
	truncated: false,
};

const DID_NOT_PRINT: PrintAdviceResult = {
	printed: false,
	truncated: false,
};

export default function printAdvice(
	item: DiagnosticAdviceItem,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	switch (item.type) {
		case "log":
			return printLog(item, opts);

		case "action":
			return printAction(item, opts);

		case "list":
			return printList(item, opts);

		case "diff":
			return printDiff(item, opts);

		case "code":
			return printCode(item, opts);

		case "command":
			return printCommand(item, opts);

		case "frame":
			return printFrame(item, opts);

		case "stacktrace":
			return printStacktrace(item, opts);

		case "inspect":
			return printInspect(item, opts);
	}
}

function printAction(
	item: DiagnosticAdviceAction,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	if (item.hidden && !opts.printer.flags.verboseDiagnostics) {
		return DID_NOT_PRINT;
	}

	opts.reporter.info(item.instruction);

	const command = serializeCLIFlags(
		{
			prefix: "",
			programName: "rome",
			commandName: item.command,
			args: item.args,
			flags: {
				...item.commandFlags,
				...item.requestFlags,
			},
		},
		{type: "none"},
	).sourceText;
	opts.reporter.command(command);
	return DID_PRINT;
}

function printCommand(
	item: DiagnosticAdviceCommand,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	opts.reporter.command(item.command);
	return DID_PRINT;
}

function printInspect(
	item: DiagnosticAdviceInspect,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;
	reporter.indent(() => {
		reporter.inspect(item.data);
	});
	return DID_PRINT;
}

function generateDiffHint(diffs: Diffs): undefined | DiagnosticAdviceItem {
	let expected = "";
	let received = "";

	for (const [type, text] of diffs) {
		switch (type) {
			case diffConstants.ADD: {
				received += text;
				break;
			}

			case diffConstants.DELETE: {
				expected += text;
				break;
			}

			case diffConstants.EQUAL: {
				expected += text;
				received += text;
				break;
			}
		}
	}

	if (expected.trim() === received.trim()) {
		return {
			type: "log",
			category: "info",
			text: "Only difference is leading and trailing whitespace",
		};
	}

	const receivedNoCRLF = removeCarriageReturn(received);
	if (expected === receivedNoCRLF) {
		return {
			type: "log",
			category: "info",
			text: "Identical except the received uses CRLF newlines, while the expected does not",
		};
	}

	const expectedNoCRLF = removeCarriageReturn(expected);
	if (received === expectedNoCRLF) {
		return {
			type: "log",
			category: "info",
			text: "Identical except the expected uses CRLF newlines, while the received does not",
		};
	}

	return undefined;
}

function printDiff(
	item: DiagnosticAdviceDiff,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {frame, truncated} = buildPatchCodeFrame(
		item,
		opts.flags.verboseDiagnostics,
	);
	if (frame === "") {
		return DID_NOT_PRINT;
	}

	opts.reporter.logAll(frame);

	const hint = generateDiffHint(item.diff);
	if (hint !== undefined) {
		opts.reporter.br();
		printAdvice(hint, opts);
		opts.reporter.br();
	}

	return {
		printed: true,
		truncated,
	};
}

function printList(
	item: DiagnosticAdviceList,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	if (item.list.length === 0) {
		return DID_NOT_PRINT;
	} else {
		const {truncated} = opts.reporter.list(
			item.list,
			{
				truncate: opts.flags.verboseDiagnostics ? undefined : 10,
				reverse: item.reverse,
				ordered: item.ordered,
			},
		);
		return {
			printed: true,
			truncated,
		};
	}
}

function printCode(
	item: DiagnosticAdviceCode,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;

	const truncated =
		!opts.flags.verboseDiagnostics && item.code.length > RAW_CODE_MAX_LENGTH;
	let code = truncated ? item.code.slice(0, RAW_CODE_MAX_LENGTH) : item.code;

	reporter.indent(() => {
		if (code === "") {
			reporter.logAll("<dim>empty input</dim>");
		} else {
			// If it's a string with only whitespace then make it obvious
			if (code.trim() === "") {
				code = showInvisibles(code);
			}

			reporter.logAll(`<nobr>${escapeMarkup(code)}</nobr>`);
		}

		if (truncated) {
			reporter.logAll(
				`<dim><number>${item.code.length - RAW_CODE_MAX_LENGTH}</number> more characters truncated</dim>`,
			);
		}
	});

	return {
		printed: true,
		truncated,
	};
}

function printFrame(
	item: DiagnosticAdviceFrame,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;
	const {marker, start, end, filename} = item.location;
	let {sourceText} = item.location;
	const path = opts.printer.createFilePath(filename);

	let cleanMarker: string = "";
	if (marker !== undefined) {
		cleanMarker = markupTag("emphasis", cleanMessage(marker));
	}

	let lines: ToLines = {
		length: 0,
		raw: [],
		highlighted: [],
	};
	if (sourceText !== undefined) {
		lines = toLines({
			path,
			input: sourceText,
			sourceType: item.location.sourceType,
			language: item.location.language,
		});
	} else if (filename !== undefined) {
		const source = opts.fileSources.get(path);
		if (source !== undefined) {
			lines = source.lines;
			sourceText = source.sourceText;
		}
	} else if (
		path.isAbsolute() &&
		opts.missingFileSources.has(path.assertAbsolute())
	) {
		lines = {
			length: 1,
			raw: ["File does not exist"],
			highlighted: ["<dim>File does not exist</dim>"],
		};
	}

	if (sourceText === undefined) {
		sourceText = "";
	}

	const frame = buildMessageCodeFrame(
		sourceText,
		lines,
		start,
		end,
		cleanMarker,
	);
	if (frame.trim() === "") {
		return DID_NOT_PRINT;
	}

	reporter.logAll(frame);
	return DID_PRINT;
}

function printStacktrace(
	item: DiagnosticAdviceStacktrace,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {diagnostic} = opts;
	const {frames} = item;

	let shownCodeFrames = 0;

	const isFirstPart = diagnostic.description.advice[0] === item;
	if (!isFirstPart) {
		const {title} = item;
		if (title !== undefined) {
			opts.reporter.info(escapeMarkup(title));
			opts.reporter.br(true);
		}
	}

	opts.reporter.processedList(
		frames,
		(reporter, frame) => {
			const {
				filename,
				object,
				suffix,
				property,
				prefix,
				line,
				column,
				language,
				sourceText: code,
			} = frame;

			const logParts = [];

			// Add prefix
			if (prefix !== undefined) {
				logParts.push(markupTag("dim", escapeMarkup(prefix)));
			}

			// Build path
			const objParts = [];
			if (object !== undefined) {
				objParts.push(markupTag("highlight", escapeMarkup(object), {i: 0}));
			}
			if (property !== undefined) {
				objParts.push(markupTag("highlight", escapeMarkup(property), {i: 1}));
			}
			if (objParts.length > 0) {
				logParts.push(objParts.join("."));
			}

			// Add suffix
			if (suffix !== undefined) {
				logParts.push(markupTag("success", escapeMarkup(suffix)));
			}

			// Add source
			if (filename !== undefined && line !== undefined && column !== undefined) {
				const header = diagnosticLocationToMarkupFilelink({
					filename,
					start: {
						index: ob1Number0Neg1,
						line,
						column,
					},
				});

				if (logParts.length === 0) {
					logParts.push(header);
				} else {
					logParts.push(`(<dim>${header}</dim>)`);
				}
			}

			reporter.logAll(logParts.join(" "));

			if (
				shownCodeFrames < 2 &&
				filename !== undefined &&
				line !== undefined &&
				column !== undefined
			) {
				const pos: Position = {
					index: ob1Number0Neg1,
					line,
					column,
				};

				const skipped = printFrame(
					{
						type: "frame",
						location: {
							language,
							filename,
							sourceType: "module",
							start: pos,
							end: pos,
							sourceText: code,
						},
					},
					{
						...opts,
						reporter,
					},
				);
				if (!skipped) {
					reporter.br(true);
					shownCodeFrames++;
				}
			}
		},
		{
			ordered: true,
			truncate: opts.flags.verboseDiagnostics ? undefined : 20,
		},
	);

	return DID_PRINT;
}

function printLog(
	item: DiagnosticAdviceLog,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;
	const {text: message, category} = item;

	if (message !== undefined) {
		switch (category) {
			case "none": {
				reporter.logAll(message);
				break;
			}

			case "warn": {
				reporter.warn(message);
				break;
			}

			case "info": {
				reporter.info(message);
				break;
			}

			case "error": {
				reporter.error(message);
				break;
			}

			default:
				throw new Error(`Unknown message item log category ${category}`);
		}
	}

	return item.compact ? DID_NOT_PRINT : DID_PRINT;
}

function cleanMessage(msg: string): string {
	msg = msg.trim();
	if (msg.endsWith(".")) {
		msg = msg.slice(0, -1);
	}
	return msg;
}
