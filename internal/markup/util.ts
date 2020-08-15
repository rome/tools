import {Consumer, consumeUnknown} from "@internal/consume";
import {MarkupFormatOptions, MarkupParsedAttributes} from "./types";
import {humanizeNumber} from "@internal/string-utils";
import {createUnknownPath} from "@internal/path";
import {ob1Coerce0, ob1Coerce1, ob1Get0, ob1Get1} from "@internal/ob1";
import {StaticMarkup} from "./escape";

export function createEmptyAttributes(): Consumer {
	return consumeUnknown({}, "parse/stringMarkup");
}

export function isSingleEscaped(
	markup: StaticMarkup,
): markup is {
	type: "MARKUP";
	parts: [string];
} {
	return (
		markup.type === "MARKUP" &&
		markup.parts.length === 1 &&
		typeof markup.parts[0] === "string"
	);
}

export function humanizeMarkupFilename(
	filename: string,
	opts: MarkupFormatOptions = {},
): string {
	if (opts.humanizeFilename !== undefined) {
		const override = opts.humanizeFilename(filename);
		if (override !== undefined) {
			return override;
		}
	}

	return createUnknownPath(filename).format(opts.cwd);
}

export function buildFileLink(
	attributes: MarkupParsedAttributes,
	opts: MarkupFormatOptions,
): {
	text: string;
	filename: string;
	line: undefined | string;
	column: undefined | string;
} {
	let filename = attributes.get("target").asString("");
	let line = attributes.get("line").asNumberOrVoid();
	let column = attributes.get("column").asNumberOrVoid();

	if (opts.normalizePosition !== undefined) {
		const pos = opts.normalizePosition(
			filename,
			line === undefined ? undefined : ob1Coerce1(line),
			column === undefined ? undefined : ob1Coerce0(column),
		);
		if (pos !== undefined) {
			filename = pos.filename;
			if (pos.line !== undefined) {
				line = ob1Get1(pos.line);
			}
			if (pos.column !== undefined) {
				column = ob1Get0(pos.column);
			}
		}
	}

	let text = humanizeMarkupFilename(filename, opts);

	if (line !== undefined) {
		text += `:${line}`;

		// Ignore a 0 column and just target the line
		if (column !== undefined && column !== 0) {
			text += `:${column}`;
		}
	}

	return {
		filename,
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
