import {Consumer, consumeUnknown} from "@internal/consume";
import {MarkupFormatOptions, MarkupParsedAttributes} from "./types";
import {humanizeNumber} from "@internal/numbers";
import {Path, createPath} from "@internal/path";
import {StaticMarkup} from "./escape";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";

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
		home: opts.home,
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
	let path: Path = createPath(attributes.get("target").required("").asString());
	let line = attributes.get("line").asOneIndexedNumberOrVoid();
	let column = attributes.get("column").asZeroIndexedNumberOrVoid();

	if (opts.normalizePosition !== undefined) {
		const pos = opts.normalizePosition(path, line, column);
		if (pos !== undefined) {
			path = pos.path;
			if (pos.line !== undefined) {
				line = pos.line;
			}
			if (pos.column !== undefined) {
				column = pos.column;
			}
		}
	}

	if (path.isRelative() && opts.cwd !== undefined) {
		path = opts.cwd.resolve(path);
	}

	return {
		path,
		text: formatFileLinkInnerText(path, opts, {line, column}),
		line: line === undefined ? undefined : String(line.valueOf()),
		column: column === undefined ? undefined : String(column.valueOf()),
	};
}

export function formatFileLinkInnerText(
	path: Path,
	opts: MarkupFormatOptions,
	pos?: Partial<Position>,
): string {
	let text = humanizeMarkupFilename(path, opts);
	if (pos === undefined) {
		return text;
	}

	const {line, column} = pos;
	if (line !== undefined) {
		text += `:${line.valueOf()}`;

		// Ignore a 0 column and just target the line
		if (column !== undefined && column.valueOf() !== 0) {
			text += `:${column.valueOf()}`;
		}
	}

	return text;
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
