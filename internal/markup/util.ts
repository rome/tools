import {Consumer, consumeUnknown} from "@internal/consume";
import {MarkupFormatOptions, MarkupParsedAttributes} from "./types";
import {OneIndexed, ZeroIndexed, humanizeNumber} from "@internal/numbers";
import {HOME_PATH, Path, createPath} from "@internal/path";
import {StaticMarkup} from "./escape";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export function createEmptyAttributes(): Consumer {
	return consumeUnknown({}, DIAGNOSTIC_CATEGORIES.parse, "romemarkup");
}

export function isSingleEscaped(markup: StaticMarkup): markup is [string] {
	return (
		Array.isArray(markup) &&
		markup.length === 1 &&
		typeof markup[0] === "string"
	);
}

export function humanizeMarkupFilename(
	path: Path,
	opts: MarkupFormatOptions = {},
): string {
	if (opts.humanizeFilename !== undefined) {
		const override = opts.humanizeFilename(path);
		if (override !== undefined) {
			return override;
		}
	}

	return path.format({
		cwd: opts.cwd,
		home: HOME_PATH,
	});
}

export function buildFileLink(
	attributes: MarkupParsedAttributes,
	opts: MarkupFormatOptions,
): {
	text: string;
	path: Path;
	line: undefined | string;
	column: undefined | string;
} {
	let path: Path = createPath(attributes.get("target").asString(""));
	let line = attributes.get("line").asNumberOrVoid();
	let column = attributes.get("column").asNumberOrVoid();

	if (opts.normalizePosition !== undefined) {
		const pos = opts.normalizePosition(
			path,
			line === undefined ? undefined : new OneIndexed(line),
			column === undefined ? undefined : new ZeroIndexed(column),
		);
		if (pos !== undefined) {
			path = pos.path;
			if (pos.line !== undefined) {
				line = pos.line.valueOf();
			}
			if (pos.column !== undefined) {
				column = pos.column.valueOf();
			}
		}
	}

	let text = humanizeMarkupFilename(path, opts);

	if (line !== undefined) {
		text += `:${line}`;

		// Ignore a 0 column and just target the line
		if (column !== undefined && column !== 0) {
			text += `:${column}`;
		}
	}

	return {
		path,
		text,
		line: line === undefined ? undefined : String(line),
		column: column === undefined ? undefined : String(column),
	};
}

export function formatApprox(attributes: MarkupParsedAttributes, value: string) {
	if (attributes.get("approx").asUnknown() === true) {
		return `~${value}`;
	} else {
		return value;
	}
}

export function formatGrammarNumber(
	attributes: MarkupParsedAttributes,
	value: string,
) {
	const num = Number(value);

	const none = attributes.get("none").asStringOrVoid();
	if (none !== undefined && num === 0) {
		return none;
	}

	const singular = attributes.get("singular").asStringOrVoid();
	if (singular !== undefined && num === 1) {
		return singular;
	}

	const plural = attributes.get("plural").asStringOrVoid();
	if (plural !== undefined) {
		return plural;
	}

	return "";
}

export function formatNumber(attributes: MarkupParsedAttributes, value: string) {
	const num = Number(value);
	const human = humanizeNumber(num);
	return formatApprox(attributes, human);
}
