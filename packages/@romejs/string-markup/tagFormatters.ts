/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MarkupFormatOptions, TagAttributes} from "./types";
import {humanizeNumber} from "@romejs/string-utils";
import {createUnknownFilePath} from "@romejs/path";

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

export function getFileLinkText(
	filename: string,
	attributes: TagAttributes,
	opts: MarkupFormatOptions,
): string {
	let text = humanizeMarkupFilename(filename, opts);

	const line = attributes.line;
	if (line !== undefined) {
		text += `:${line}`;

		const column = attributes.column;
		// Ignore a 0 column and just target the line
		if (column !== undefined && column !== "0") {
			text += `:${column}`;
		}
	}

	return text;
}

export function getFileLinkFilename(
	attributes: TagAttributes,
	opts: MarkupFormatOptions,
): string {
	let filename = attributes.target || "";
	if (opts.normalizeFilename !== undefined) {
		filename = opts.normalizeFilename(filename);
	}
	return filename;
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
