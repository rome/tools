/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	MarkupColor,
	MarkupFormatOptions,
	MarkupTokenType,
	TagAttributes,
} from "../types";
import {humanizeNumber} from "@romejs/string-utils";
import {createUnknownFilePath} from "@romejs/path";
import {ob1Coerce0, ob1Coerce1} from "@romejs/ob1";

export function normalizeTokenType(
	type: undefined | string,
): undefined | MarkupTokenType {
	switch (type) {
		case "boolean":
		case "keyword":
		case "number":
		case "regex":
		case "string":
		case "comment":
		case "operator":
		case "punctuation":
		case "variable":
		case "attr-name":
		case "function":
		case "attr-value":
		case "attr-equals":
		case "tag":
			return type;

		default:
			return undefined;
	}
}

export function normalizeColor(
	color: undefined | string,
): undefined | MarkupColor {
	switch (color) {
		case "black":
		case "brightBlack":
		case "red":
		case "brightRed":
		case "green":
		case "brightGreen":
		case "yellow":
		case "brightYellow":
		case "blue":
		case "brightBlue":
		case "magenta":
		case "brightMagenta":
		case "cyan":
		case "brightCyan":
		case "white":
		case "brightWhite":
			return color;

		default:
			return undefined;
	}
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
	attributes: TagAttributes,
	opts: MarkupFormatOptions,
): {
	text: string;
	filename: string;
	line: undefined | string;
	column: undefined | string;
} {
	let filename = attributes.target || "";
	let line = attributes.line;
	let column = attributes.column;

	if (opts.normalizePosition !== undefined) {
		const pos = opts.normalizePosition(
			filename,
			line === undefined ? undefined : ob1Coerce1(Number(line)),
			column === undefined ? undefined : ob1Coerce0(Number(column)),
		);
		if (pos !== undefined) {
			filename = pos.filename;
			line = String(pos.line);
			column = String(pos.column);
		}
	}

	let text = humanizeMarkupFilename(filename, opts);

	if (line !== undefined) {
		text += `:${line}`;

		// Ignore a 0 column and just target the line
		if (column !== undefined && column !== "0") {
			text += `:${column}`;
		}
	}

	return {filename, text, line, column};
}

export function formatApprox(attributes: TagAttributes, value: string) {
	if (attributes.approx === "true") {
		return `~${value}`;
	} else {
		return value;
	}
}

export function formatGrammarNumber(attributes: TagAttributes, value: string) {
	const num = Number(value);

	const none = attributes.none;
	if (none !== undefined && num === 0) {
		return none;
	}

	const singular = attributes.singular;
	if (singular !== undefined && num === 1) {
		return singular;
	}

	const plural = attributes.plural;
	if (plural !== undefined) {
		return plural;
	}

	return "";
}

export function formatNumber(attributes: TagAttributes, value: string) {
	const num = Number(value);
	const human = humanizeNumber(num);
	const humanWithApprox = formatApprox(attributes, human);
	return humanWithApprox;
}
