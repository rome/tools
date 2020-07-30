/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyMarkup, MarkupLinesAndWidth, readMarkup} from "@internal/markup";
import {GridOutputFormat, UserGridOptions} from "./types";
import Grid from "./Grid";
import {ob1Get1} from "@internal/ob1";

export function renderGrid(
	safe: AnyMarkup,
	opts: UserGridOptions = {},
	format: GridOutputFormat,
): MarkupLinesAndWidth {
	const input = readMarkup(safe);
	const grid = new Grid({
		...opts,
		sourceText: input,
		view: {},
	});
	grid.drawChildren(grid.parse(input, undefined), []);
	return {
		width: ob1Get1(grid.getWidth()),
		lines: grid.getLines(format),
	};
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
