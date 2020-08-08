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
import {createJSParser} from "./parser";
import "./tokenizer/context";

export {default as CommentsConsumer} from "./CommentsConsumer";

export function parseJS(userOptions: JSParserUserOptions): JSRoot {
	const options: JSParserOptions = normalizeOptions(userOptions);
	return createJSParser(options).parse();
}

export function tokenizeJS(userOptions: JSParserUserOptions): Array<PublicToken> {
	const options: JSParserOptions = normalizeOptions(userOptions);
	const parser = createJSParser({...options, tokens: true});
	parser.parse();

	let tokens: Array<PublicToken> = [];

	for (const token of parser.state.tokens) {
		tokens.push({
			type: token.type.label,
			start: parser.getIndexFromPosition(token.loc.start, token.loc.filename),
			end: parser.getIndexFromPosition(token.loc.end, token.loc.filename),
		});
	}

	return tokens;
}

export {Token};

export {tokTypes};

export {keywords as keywordTokTypes} from "./tokenizer/types";
