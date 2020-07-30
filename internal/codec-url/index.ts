/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consume, consumeUnknown} from "@internal/consume";
import url = require("url");

import {ob1Coerce0, ob1Number0, ob1Number1} from "@internal/ob1";

export type ConsumableUrl = {
	path: Consumer;
	query: Consumer;
};

export function consumeUrl(rawUrl: string): ConsumableUrl {
	const parts = url.parse(rawUrl, true);

	const query = consumeUnknown({...parts.query}, "parse/url/query");

	const path = consume({
		value: parts.pathname,
		context: {
			category: "parse/url",
			getDiagnosticLocation() {
				return {
					language: "url",
					mtime: undefined,
					sourceText: rawUrl,
					filename: "url",
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

	return {query, path};
}
