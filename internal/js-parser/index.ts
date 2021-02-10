/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSRoot} from "@internal/ast";
import {
	JSParserOptions,
	JSParserUserOptions,
	normalizeOptions,
} from "./options";
import {PublicToken, Token} from "./tokenizer/index";
import {types as tokTypes} from "./tokenizer/types";
import "./tokenizer/context";
import {jsParser, createMeta, parseRoot} from "./parser";

export {default as CommentsConsumer} from "./CommentsConsumer";

export function parseJS(userOptions: JSParserUserOptions): JSRoot {
	const options = normalizeOptions(userOptions);
	const meta = createMeta(options);
	const parser = jsParser.create(options, meta);
	return parseRoot(parser);
}

export function tokenizeJS(userOptions: JSParserUserOptions): PublicToken[] {
	const options: JSParserOptions = {
		...normalizeOptions(userOptions),
		tokens: true,
	};
	const meta = createMeta(options);
	const parser = jsParser.create(options, meta);
	parseRoot(parser);

	let tokens: PublicToken[] = [];

	for (const token of parser.state.tokens) {
		tokens.push({
			type: token.type.label,
			start: parser.getIndexFromPosition(token.loc.start, token.loc.path),
			end: parser.getIndexFromPosition(token.loc.end, token.loc.path),
		});
	}

	return tokens;
}

export {Token};

export {tokTypes};

export {keywords as keywordTokTypes} from "./tokenizer/types";
