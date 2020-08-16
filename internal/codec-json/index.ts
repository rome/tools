/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSONParserOptions, JSONValue, RJSONCommentMap, Tokens} from "./types";
import {createJSONParser, parseJSONExtra} from "./parse";
import {Consumer, consume, consumeUnknown} from "@internal/consume";
import {stringifyRootConsumer} from "./stringify";
import {TokenValues} from "@internal/parser-core";

export {
	JSONArray,
	JSONObject,
	JSONParserOptions,
	JSONPropertyValue,
	JSONValue,
	RJSONCommentMap,
} from "./types";

export type ConsumeJSONResult = {
	hasExtensions: boolean;
	consumer: Consumer;
	comments: RJSONCommentMap;
};

export function consumeJSON(opts: JSONParserOptions): Consumer {
	return consumeJSONExtra(opts).consumer;
}

export function consumeJSONExtra(opts: JSONParserOptions): ConsumeJSONResult {
	const {value, context, hasExtensions, comments, path} = parseJSONExtra(opts);

	return {
		hasExtensions,
		consumer: consume({
			filePath: path,
			context,
			objectPath: [],
			value,
			parent: undefined,
		}),
		comments,
	};
}

export function parseJSON(opts: JSONParserOptions): JSONValue {
	return parseJSONExtra(opts).value;
}

export function tokenizeJSON(
	opts: JSONParserOptions,
): Array<TokenValues<Tokens>> {
	return createJSONParser(opts).tokenizeAll();
}

export function stringifyRJSONFromConsumer(
	opts: {
		consumer: Consumer;
		comments: RJSONCommentMap;
	},
): string {
	return stringifyRootConsumer(opts.consumer, opts.comments);
}

export function stringifyJSONExtra(res: ConsumeJSONResult): string {
	if (res.hasExtensions) {
		return stringifyRJSONFromConsumer(res);
	} else {
		return stringifyJSON(res.consumer.asUnknown());
	}
}

export function stringifyRJSON(
	value: unknown,
	comments: RJSONCommentMap = new Map(),
): string {
	return stringifyRootConsumer(consumeUnknown(value, "parse/json"), comments);
}

export function stringifyJSON(value: unknown, spaces: boolean = false): string {
	return JSON.stringify(value, null, spaces ? "  " : "\t");
}
