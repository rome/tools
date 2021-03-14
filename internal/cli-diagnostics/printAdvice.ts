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
	Markup,
	isEmptyMarkup,
	joinMarkup,
	markup,
	markupTag,
	normalizeMarkup,
	readMarkup,
} from "@internal/markup";
import {DiagnosticsPrinterFlags} from "./types";
import DiagnosticsPrinter, {DiagnosticsPrinterFileSources} from "./DiagnosticsPrinter";
import {MixedPathSet, createUIDPath} from "@internal/path";
import {MAX_CODE_LENGTH, MAX_CODE_LINES, MAX_LOG_LENGTH} from "./constants";
import {CompressedDiff, DiffTypes} from "@internal/string-diff";
import {removeCarriageReturn} from "@internal/string-utils";
import {serializeCLIFlags} from "@internal/cli-flags";
import {inferDiagnosticLanguageFromPath} from "@internal/core/common/file-handlers";

type AdvicePrintOptions = {
	printer: DiagnosticsPrinter;
	flags: DiagnosticsPrinterFlags;
	missingFileSources: MixedPathSet;
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
	advice: DiagnosticAdvice[],
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	let truncated = false;
	let printed = false;

	for (const item of advice) {
		const res = printAdviceItem(item, opts);
		if (res.printed) {
			printed = true;
			opts.reporter.br();
		}
		if (res.truncated) {
			truncated = true;
		}
	}

	return {
		truncated,
		printed,
	};
}

function printAdviceItem(
	item: DiagnosticAdvice,
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
	let truncated = false;

	const {reporter} = opts;
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
	opts.reporter.info(
		markup`<emphasis>Command Suggestion:</emphasis> ${item.description}`,
	);

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

function generateDiffHint(diffs: CompressedDiff[]): undefined | DiagnosticAdvice {
	let expected = "";
	let received = "";

	for (const [type, text] of diffs) {
		switch (type) {
			case DiffTypes.INSERT: {
				received += text;
				break;
			}

			case DiffTypes.DELETE: {
				expected += text;
				break;
			}

			case DiffTypes.EQUAL: {
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
		opts.flags.truncateDiagnostics,
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
				truncate: opts.flags.truncateDiagnostics ? 10 : undefined,
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

function printTruncatedCharacters(reporter: Reporter, chars: number) {
	reporter.log(markup`<dim>${chars} more characters truncated</dim>`);
}

function printCode(
	item: DiagnosticAdviceCode,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;

	const shouldTruncate =
		opts.flags.truncateDiagnostics && item.truncate !== false;

	const didTruncateCharacters =
		shouldTruncate && item.sourceText.length > MAX_CODE_LENGTH;
	let code = didTruncateCharacters
		? item.sourceText.slice(0, MAX_CODE_LENGTH)
		: item.sourceText;

	const {frame, truncated: didTruncateLines} = buildCodeFrame({
		type: "all",
		truncateLines: shouldTruncate ? MAX_CODE_LINES : undefined,
		lines: toLines({
			input: code,
			path: createUIDPath("inline"),
			sourceTypeJS: item.sourceTypeJS,
			language: item.language,
			highlight: opts.printer.shouldHighlight(),
		}),
	});
	if (isEmptyMarkup(frame)) {
		return DID_NOT_PRINT;
	}

	reporter.log(frame);

	if (didTruncateCharacters) {
		printTruncatedCharacters(reporter, item.sourceText.length - MAX_CODE_LENGTH);
	}

	return {
		printed: true,
		truncated: didTruncateLines || didTruncateCharacters,
	};
}

function printFrame(
	item: DiagnosticAdviceFrame,
	opts: AdvicePrintOptions,
): PrintAdviceResult {
	const {reporter} = opts;
	const {marker, start, end} = item.location;
	let {sourceText} = item.location;
	const path = opts.printer.normalizePath(item.location.path);

	let lines: ToLines = [];
	if (sourceText !== undefined) {
		lines = toLines({
			path,
			input: sourceText,
			sourceTypeJS: item.location.sourceTypeJS,
			language: inferDiagnosticLanguageFromPath(path, item.location.language),
			highlight: opts.printer.shouldHighlight(),
		});
	} else if (path !== undefined) {
		const source = opts.fileSources.get(path);
		if (source === undefined) {
			if (opts.missingFileSources.has(path)) {
				return printLog(
					{
						type: "log",
						category: "warn",
						text: markup`Cannot render frame as ${diagnosticLocationToMarkupFilelink(
							item.location,
						)} does not exist`,
					},
					opts,
				);
			} else {
				return DID_NOT_PRINT;
			}
		} else {
			lines = source.lines;
			sourceText = source.sourceText;
		}
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
			opts.reporter.info(title);
			opts.reporter.br({force: true});
		}
	}

	opts.reporter.processedList(
		frames,
		(reporter, frame) => {
			const {
				path,
				object,
				suffix,
				property,
				language,
				prefix,
				line,
				column,
				sourceText: code,
			} = frame;

			const logParts: Markup[] = [];

			// Add prefix
			if (prefix !== undefined) {
				logParts.push(markupTag("dim", markup`${prefix}`));
			}

			// Build path
			const objParts: Markup[] = [];
			if (object !== undefined) {
				objParts.push(markupTag("highlight", markup`${object}`, {i: 0}));
			}
			if (property !== undefined) {
				objParts.push(markupTag("highlight", markup`${property}`, {i: 1}));
			}
			if (objParts.length > 0) {
				logParts.push(joinMarkup(objParts, markup`.`));
			}

			// Add suffix
			if (suffix !== undefined) {
				logParts.push(markupTag("success", markup`${suffix}`));
			}

			// Add source
			if (path !== undefined && line !== undefined && column !== undefined) {
				let header = diagnosticLocationToMarkupFilelink({
					path,
					language,
					start: {
						line,
						column,
					},
				});

				if (logParts.length > 0) {
					header = markup`(${header})`;
				}

				logParts.push(markup`<dim>${header}</dim>`);
			}

			reporter.log(joinMarkup(logParts, markup` `));

			// A code frame will always be displayed if it's been marked as important on the stackframe advice or if it
			// refers to the diagnostic
			const isImportantStackFrame =
				path !== undefined &&
				(path.equal(diagnostic.location.path) ||
				(item.importantPaths !== undefined && item.importantPaths.has(path)));
			const shouldShowCodeFrame = isImportantStackFrame || shownCodeFrames < 2;

			if (
				shouldShowCodeFrame &&
				path !== undefined &&
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
							path,
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
			truncate: opts.flags.truncateDiagnostics ? 20 : undefined,
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
		printTruncatedCharacters(reporter, truncatedLength);
	}

	return {
		printed: true,
		truncated,
	};
}
