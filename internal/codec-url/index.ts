/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consume, consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import url = require("url");
import {ob1Coerce0, ob1Number0, ob1Number1} from "@internal/ob1";
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
						index: ob1Number0,
						line: ob1Number1,
						column: ob1Number0,
					},
					end: {
						index: ob1Coerce0(rawUrl.length - 1),
						line: ob1Number1,
						column: ob1Coerce0(rawUrl.length - 1),
					},
				};
			},
		},
	});

	return {query, href};
}
