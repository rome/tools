import {Consumer, consumeUnknown} from "@romefrontend/consume";
import {MarkupFormatOptions, MarkupParsedAttributes} from "./types";
import {humanizeNumber} from "@romefrontend/string-utils";
import {createUnknownFilePath} from "@romefrontend/path";
import {ob1Coerce0, ob1Coerce1, ob1Get0, ob1Get1} from "@romefrontend/ob1";

export function createEmptyAttributes(): Consumer {
	return consumeUnknown({}, "parse/stringMarkup");
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

	return createUnknownFilePath(filename).format(opts.cwd);
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
	const humanWithApprox = formatApprox(attributes, human);
	return humanWithApprox;
}
