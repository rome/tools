/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consume, consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import url = require("url");
import {OneIndexed, ZeroIndexed} from "@internal/math";
import {AnyPath, UNKNOWN_PATH} from "@internal/path";

export type ConsumableUrl = {
	href: Consumer;
	query: Consumer;
};

export function consumeUrl(
	rawUrl: string,
	path: AnyPath = UNKNOWN_PATH,
): ConsumableUrl {
	const parts = url.parse(rawUrl, true);

	const query = consumeUnknown(
		{...parts.query},
		DIAGNOSTIC_CATEGORIES.parse,
		"urlquery",
	);

	const href = consume({
		path,
		value: parts.pathname,
		context: {
			category: DIAGNOSTIC_CATEGORIES.parse,
			categoryValue: "url",
			getDiagnosticLocation() {
				return {
					language: "url",
					mtime: undefined,
					sourceText: rawUrl,
					path: href.path,
					start: {
						index: new ZeroIndexed(),
						line: new OneIndexed(),
						column: new ZeroIndexed(),
					},
					end: {
						index: new ZeroIndexed(rawUrl.length - 1),
						line: new OneIndexed(),
						column: new ZeroIndexed(rawUrl.length - 1),
					},
				};
			},
		},
	});

	return {query, href};
}
