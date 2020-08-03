/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyMarkup,
	MarkupLinesAndWidth,
	isSingleEscaped,
	joinMarkupLines,
	readMarkup,
	serializeLazyMarkup,
} from "@internal/markup";
import {GridOutputFormat, UserGridOptions} from "./types";
import Grid from "./Grid";
import {ob1Get1} from "@internal/ob1";
import {splitChars, splitLines} from "@internal/string-utils";

export function renderGrid(
	safeMaybeLazy: AnyMarkup,
	opts: UserGridOptions = {},
	format: GridOutputFormat,
): MarkupLinesAndWidth {
	const safe = serializeLazyMarkup(safeMaybeLazy);

	// Optimization for rendering a single escaped string with no columns
	if (opts.columns === undefined && isSingleEscaped(safe)) {
		let line = safe.parts[0];

		if (opts.convertTabs) {
			// TODO make the tab width customizable in userConfig
			line = line.replace(/\t/g, " ".repeat(2));
		}

		const lines = splitLines(line);
		const width = Math.max(...lines.map((line) => splitChars(line).length));

		return {
			width,
			lines,
		};
	}

	const input = readMarkup(safe);
	const grid = new Grid({
		...opts,
		sourceText: input,
		view: {},
	});
	grid.drawChildren(grid.parse(safe, undefined), []);
	return {
		width: ob1Get1(grid.getWidth()),
		lines: grid.getLines(format),
	};
}

export function markupToJoinedPlainText(
	input: AnyMarkup,
	opts: UserGridOptions = {},
): string {
	return joinMarkupLines(markupToPlainText(input, opts));
}

export function markupToPlainText(
	input: AnyMarkup,
	opts: UserGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "none");
}

export function markupToAnsi(
	input: AnyMarkup,
	opts: UserGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "ansi");
}

export function markupToHtml(
	input: AnyMarkup,
	opts: UserGridOptions = {},
): MarkupLinesAndWidth {
	return renderGrid(input, opts, "html");
}
