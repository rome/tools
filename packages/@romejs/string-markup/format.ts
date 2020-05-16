/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Children,
	MarkupFormatGridOptions,
	MarkupFormatNormalizeOptions,
	TagNode,
} from './types';
import {parseMarkup} from './parse';
import {escapeMarkup} from './escape';
import Grid from './Grid';
import {ob1Get1} from '@romejs/ob1';
import {
	formatGrammarNumber,
	getFileLinkFilename,
	getFileLinkText,
} from './tagFormatters';

function buildTag(
	tag: TagNode,
	inner: string,
	opts: MarkupFormatNormalizeOptions,
): string {
	let {attributes} = tag;

	switch (tag.name) {
		case // Normalize filename of <filelink target>
		'filelink': {
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
		case 'grammarNumber':
			return formatGrammarNumber(attributes, inner);
	}

	let open = `<${tag.name}`;

	// Print attributes
	for (const key in attributes) {
		const value = attributes[key];
		if (value === undefined) {
			continue;
		}

		if (value === 'true') {
			open += ` ${key}`;
		} else {
			const escapedValue = escapeMarkup(value);
			open += ` ${key}="${escapedValue}"`;
		}
	}

	if (inner === '') {
		return `${open} />`;
	} else {
		return `${open}>${inner}</${tag.name}>`;
	}
}

function normalizeMarkupChildren(
	children: Children,
	opts: MarkupFormatNormalizeOptions,
): string {
	// Sometimes we'll populate the inner text of a tag with no children
	if (children.length === 0) {
		return '';
	}

	let buff = '';
	for (const child of children) {
		if (child.type === 'Text') {
			buff += escapeMarkup(child.value);
		} else if (child.type === 'Tag') {
			const inner = normalizeMarkupChildren(child.children, opts);
			buff += buildTag(child, inner, opts);
		} else {
			throw new Error('Unknown child node type');
		}
	}
	return buff;
}

export function markupToPlainTextString(
	input: string,
	opts: MarkupFormatGridOptions = {},
): string {
	return markupToPlainText(input, opts).lines.join('\n');
}

export function markupToPlainText(
	input: string,
	opts: MarkupFormatGridOptions = {},
): MarkupLinesAndWidth {
	const grid = new Grid(opts);
	grid.drawRoot(parseMarkup(input));
	return {
		width: ob1Get1(grid.getWidth()),
		lines: grid.getLines(),
	};
}

export type MarkupLinesAndWidth = {
	width: number;
	lines: Array<string>;
};

export function markupToAnsi(
	input: string,
	opts: MarkupFormatGridOptions = {},
): MarkupLinesAndWidth {
	const grid = new Grid(opts);

	grid.drawRoot(parseMarkup(input));

	return {
		width: ob1Get1(grid.getWidth()),
		lines: grid.getFormattedLines(),
	};
}

export function normalizeMarkup(
	input: string,
	opts: MarkupFormatNormalizeOptions = {},
): string {
	return normalizeMarkupChildren(parseMarkup(input), opts);
}
