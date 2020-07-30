/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSONParserOptions, JSONValue, PathToComments, Tokens} from "./types";
import {createJSONParser} from "./parse";
import {Consumer, consume, consumeUnknown} from "@internal/consume";
import {stringifyRootConsumer} from "./stringify";
import {TokenValues} from "@internal/parser-core";

export {
	JSONArray,
	JSONObject,
	JSONParserOptions,
	JSONPropertyValue,
	JSONValue,
} from "./types";

export type ConsumeJSONResult = {
	hasExtensions: boolean;
	consumer: Consumer;
	comments: PathToComments;
};

export function consumeJSON(opts: JSONParserOptions): Consumer {
	return consumeJSONExtra(opts).consumer;
}

export function consumeJSONExtra(opts: JSONParserOptions): ConsumeJSONResult {
	const parser = createJSONParser(opts);
	const {value, context} = parser.parse();

	return {
		hasExtensions: parser.hasExtensions,
		consumer: consume({
			filePath: parser.path,
			context,
			objectPath: [],
			value,
			parent: undefined,
		}),
		comments: parser.pathToComments,
	};
}

export function parseJSON(opts: JSONParserOptions): JSONValue {
	return createJSONParser(opts).parse().value;
}

export function tokenizeJSON(
	opts: JSONParserOptions,
): Array<TokenValues<Tokens>> {
	return createJSONParser(opts).tokenizeAll();
}

export function stringifyRJSONFromConsumer(
	opts: {
		consumer: Consumer;
		comments: PathToComments;
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

export function stringifyRJSON(value: unknown): string {
	return stringifyRootConsumer(consumeUnknown(value, "parse/json"), new Map());
}

export function stringifyJSON(value: unknown): string {
	return JSON.stringify(value, null, "\t");
}
