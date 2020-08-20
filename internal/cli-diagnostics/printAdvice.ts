/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@internal/cli-reporter";
import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticAdviceAction,
	DiagnosticAdviceCode,
	DiagnosticAdviceCommand,
	DiagnosticAdviceDiff,
	DiagnosticAdviceFrame,
	DiagnosticAdviceGroup,
	DiagnosticAdviceInspect,
	DiagnosticAdviceItem,
	DiagnosticAdviceList,
	DiagnosticAdviceLog,
	DiagnosticAdviceStacktrace,
	diagnosticLocationToMarkupFilelink,
} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";
import {ToLines, toLines} from "./utils";
import buildPatchCodeFrame from "./buildPatchCodeFrame";
import buildCodeFrame from "./buildCodeFrame";
import {
	AnyMarkups,
	concatMarkup,
	isEmptyMarkup,
	markup,
	markupTag,
	normalizeMarkup,
	readMarkup,
} from "@internal/markup";
import {DiagnosticsPrinterFlags} from "./types";
import DiagnosticsPrinter, {DiagnosticsPrinterFileSources} from "./DiagnosticsPrinter";
import {UnknownPathSet, createUnknownPath} from "@internal/path";
import {MAX_CODE_LENGTH, MAX_CODE_LINES, MAX_LOG_LENGTH} from "./constants";
import {Diffs, diffConstants} from "@internal/string-diff";
import {removeCarriageReturn} from "@internal/string-utils";
import {serializeCLIFlags} from "@internal/cli-flags";
import {inferDiagnosticLanguageFromFilename} from "@internal/core/common/file-handlers";

type AdvicePrintOptions = {
	printer: DiagnosticsPrinter;
	flags: DiagnosticsPrinterFlags;
	missingFileSources: UnknownPathSet;
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

export function printAdvice(
	advice: DiagnosticAdvice,
	opts: AdvicePrintOptions,
): {
	truncated: boolean;
} {
	let truncated = false;

	for (const item of advice) {
		const res = printAdviceItem(item, opts);
		if (res.printed) {
			opts.reporter.br();
		}
		if (res.truncated) {
			truncated = true;
		}
	}

	return {truncated};
}

export function printAdviceItem(
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

		case "group":
			return printGroup(item, opts);
	}
}

function printGroup(
	item: DiagnosticAdviceGroup,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;

	let truncated = false;

	reporter.log(markup`<emphasis>${item.title}</emphasis>`);
	reporter.br();
	reporter.indentSync(() => {
		({truncated} = printAdvice(item.advice, opts));
	});

	return {
		printed: true,
		truncated,
	};
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
			programName: "rome",
			commandName: item.command,
			args: item.args ?? [],
			flags: {
				...item.commandFlags,
				...item.requestFlags,
			},
		},
		"none",
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
	reporter.indentSync(() => {
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
			text: markup`Only difference is leading and trailing whitespace`,
		};
	}

	const receivedNoCRLF = removeCarriageReturn(received);
	if (expected === receivedNoCRLF) {
		return {
			type: "log",
			category: "info",
			text: markup`Identical except the received uses CRLF newlines, while the expected does not`,
		};
	}

	const expectedNoCRLF = removeCarriageReturn(expected);
	if (received === expectedNoCRLF) {
		return {
			type: "log",
			category: "info",
			text: markup`Identical except the expected uses CRLF newlines, while the received does not`,
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
		opts.flags.verboseDiagnostics !== false,
	);
	if (isEmptyMarkup(frame)) {
		return DID_NOT_PRINT;
	}

	opts.reporter.log(frame);

	const hint = generateDiffHint(item.diff);
	if (hint !== undefined) {
		opts.reporter.br();
		printAdviceItem(hint, opts);
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

function printTruncated(reporter: Reporter, chars: number) {
	reporter.log(markup`<dim>${chars} more characters truncated</dim>`);
}

function printCode(
	item: DiagnosticAdviceCode,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;

	const truncatedLength =
		!opts.flags.verboseDiagnostics && item.sourceText.length > MAX_CODE_LENGTH;
	let code = truncatedLength
		? item.sourceText.slice(0, MAX_CODE_LENGTH)
		: item.sourceText;

	const {frame, truncated: truncatedLines} = buildCodeFrame({
		type: "all",
		truncateLines: MAX_CODE_LINES,
		lines: toLines({
			input: code,
			path: createUnknownPath("inline"),
			sourceTypeJS: item.sourceTypeJS,
			language: item.language,
			highlight: opts.printer.shouldHighlight(),
		}),
	});
	if (isEmptyMarkup(frame)) {
		return DID_NOT_PRINT;
	}

	reporter.log(frame);

	if (truncatedLength) {
		printTruncated(reporter, item.sourceText.length - MAX_CODE_LENGTH);
	}

	return {
		printed: true,
		truncated: truncatedLines || truncatedLength,
	};
}

function printFrame(
	item: DiagnosticAdviceFrame,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;
	const {marker, start, end, filename} = item.location;
	let {sourceText} = item.location;
	const path =
		filename === undefined
			? createUnknownPath("unknown")
			: opts.printer.createFilePath(filename);

	let lines: ToLines = [];
	if (sourceText !== undefined) {
		lines = toLines({
			path,
			input: sourceText,
			sourceTypeJS: item.location.sourceTypeJS,
			language: inferDiagnosticLanguageFromFilename(
				path,
				item.location.language,
			),
			highlight: opts.printer.shouldHighlight(),
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
		return printLog(
			{
				type: "log",
				category: "warn",
				text: markup`File ${path} does not exist`,
			},
			opts,
		);
	}

	if (sourceText === undefined) {
		sourceText = "";
	}

	const {frame, truncated} = buildCodeFrame({
		type: "pointer",
		lines,
		start,
		end,
		markerMessage: marker,
	});
	if (isEmptyMarkup(frame)) {
		return DID_NOT_PRINT;
	}

	reporter.log(frame);
	return {
		printed: true,
		truncated,
	};
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
			opts.reporter.info(markup`${title}`);
			opts.reporter.br({force: true});
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
				language,
				prefix,
				line,
				column,
				sourceText: code,
			} = frame;

			const logParts: AnyMarkups = [];

			// Add prefix
			if (prefix !== undefined) {
				logParts.push(markupTag("dim", markup`${prefix}`));
			}

			// Build path
			const objParts: AnyMarkups = [];
			if (object !== undefined) {
				objParts.push(markupTag("highlight", markup`${object}`, {i: 0}));
			}
			if (property !== undefined) {
				objParts.push(markupTag("highlight", markup`${property}`, {i: 1}));
			}
			if (objParts.length > 0) {
				logParts.push(concatMarkup(objParts, markup`.`));
			}

			// Add suffix
			if (suffix !== undefined) {
				logParts.push(markupTag("success", markup`${suffix}`));
			}

			// Add source
			if (filename !== undefined && line !== undefined && column !== undefined) {
				const header = diagnosticLocationToMarkupFilelink({
					filename,
					language,
					start: {
						line,
						column,
					},
				});

				if (logParts.length === 0) {
					logParts.push(header);
				} else {
					logParts.push(markup`<dim>(${header})</dim>`);
				}
			}

			reporter.log(concatMarkup(logParts, markup` `));

			// A code frame will always be displayed if it's been marked as important on the stackframe advice or if it
			// refers to the diagnostic
			const isImportantStackFrame =
				filename !== undefined &&
				(filename === diagnostic.location.filename ||
				(item.importantFilenames !== undefined &&
				item.importantFilenames.includes(filename)));
			const shouldShowCodeFrame = isImportantStackFrame || shownCodeFrames < 2;

			if (
				shouldShowCodeFrame &&
				filename !== undefined &&
				line !== undefined &&
				column !== undefined
			) {
				const pos: Position = {
					line,
					column,
				};

				const frame = printFrame(
					{
						type: "frame",
						location: {
							filename,
							language,
							sourceTypeJS: "module",
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
				if (frame.printed && !isImportantStackFrame) {
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
	const {category} = item;
	let {text} = item;

	let truncated = false;
	let truncatedLength = 0;
	if (readMarkup(text).length > MAX_LOG_LENGTH) {
		({truncated, text, truncatedLength} = normalizeMarkup(
			text,
			{},
			MAX_LOG_LENGTH,
		));
	}

	if (text !== undefined) {
		switch (category) {
			case "none": {
				reporter.log(text);
				break;
			}

			case "warn": {
				reporter.warn(text);
				break;
			}

			case "info": {
				reporter.info(text);
				break;
			}

			case "error": {
				reporter.error(text);
				break;
			}

			default:
				throw new Error(`Unknown message item log category ${category}`);
		}
	}

	if (truncated) {
		printTruncated(reporter, truncatedLength);
	}

	return {
		printed: !item.compact,
		truncated,
	};
}
