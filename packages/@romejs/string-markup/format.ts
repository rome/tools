/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Children,
	GridOutputFormat,
	MarkupFormatGridOptions,
	MarkupFormatNormalizeOptions,
	MarkupLinesAndWidth,
	TagNode,
} from "./types";
import {parseMarkup} from "./parse";
import {escapeMarkup} from "./escape";
import Grid from "./grid/Grid";
import {ob1Get1} from "@romejs/ob1";
import {sliceEscaped} from "@romejs/string-utils";
import {
	formatGrammarNumber,
	getFileLinkFilename,
	getFileLinkText,
} from "./grid/tagFormatters";

function buildTag(
	tag: TagNode,
	inner: string,
	opts: MarkupFormatNormalizeOptions,
): string {
	let {attributes} = tag;

	switch (tag.name) {
		case // Normalize filename of <filelink target>
		"filelink": {
			// Clone
			attributes = {...attributes};

			const filename = getFileLinkFilename(attributes, opts);
			const text = getFileLinkText(filename, attributes, opts);
			attributes.target = filename;
			if (opts.stripPositions) {
				attributes.line = undefined;
				attributes.column = undefined;
			}
			inner = text;
			break;
		}

		// We don't technically need to normalize this but it's one less tag to have to support
		// if other tools need to consume it
		case "grammarNumber":
			return formatGrammarNumber(attributes, inner);
	}

	let open = `<${tag.name}`;

	// Print attributes
	for (const key in attributes) {
		const value = attributes[key];
		if (value === undefined) {
			continue;
		}

		if (value === "true") {
			open += ` ${key}`;
		} else {
			const escapedValue = escapeMarkup(value);
			open += ` ${key}="${escapedValue}"`;
		}
	}

	if (inner === "") {
		return `${open} />`;
	} else {
		return `${open}>${inner}</${tag.name}>`;
	}
}

function normalizeMarkupChildren(
	children: Children,
	opts: MarkupFormatNormalizeOptions,
	remainingChars: number,
): {
	textLength: number;
	text: string;
} {
	// Sometimes we'll populate the inner text of a tag with no children
	if (children.length === 0) {
		return {text: "", textLength: 0};
	}

	let textLength = 0;

	let buff = "";
	for (const child of children) {
		if (child.type === "Text") {
			let text = escapeMarkup(child.value);
			textLength += text.length;
			const isVisible = remainingChars > 0;
			if (text.length > remainingChars) {
				text = sliceEscaped(text, remainingChars);
			}
			remainingChars -= text.length;
			if (isVisible) {
				buff += text;
			}
		} else if (child.type === "Tag") {
			const inner = normalizeMarkupChildren(
				child.children,
				opts,
				remainingChars,
			);

			if (remainingChars > 0) {
				buff += buildTag(child, inner.text, opts);
			}
			textLength += inner.textLength;
			remainingChars -= inner.textLength;
		} else {
			throw new Error("Unknown child node type");
		}
	}

	return {
		text: buff,
		textLength,
	};
}

function renderGrid(
	input: string,
	opts: MarkupFormatGridOptions = {},
	format: GridOutputFormat,
): MarkupLinesAndWidth {
	const grid = new Grid(opts);
	grid.drawRoot(parseMarkup(input));
	return {
		width: ob1Get1(grid.getWidth()),
		lines: grid.getLines(format),
	};
}

export function markupToPlainTextString(
	input: string,
	opts: MarkupFormatGridOptions = {},
): string {
	return markupToPlainText(input, opts).lines.join("\n");
}

export function markupToPlainText(
	input: string,
	opts: MarkupFormatGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "none");
}

export function markupToAnsi(
	input: string,
	opts: MarkupFormatGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "ansi");
}

export function markupToHtml(
	input: string,
	opts: MarkupFormatGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "html");
}

export function normalizeMarkup(
	input: string,
	opts: MarkupFormatNormalizeOptions = {},
	maxLength: number = Infinity,
): {
	visibleTextLength: number;
	truncatedLength: number;
	textLength: number;
	text: string;
	truncated: boolean;
} {
	const {textLength, text} = normalizeMarkupChildren(
		parseMarkup(input),
		opts,
		maxLength,
	);

	const isTruncated = textLength > maxLength;

	return {
		textLength,
		text,
		truncated: isTruncated,
		visibleTextLength: isTruncated ? maxLength : textLength,
		truncatedLength: isTruncated ? textLength - maxLength : 0,
	};
}
