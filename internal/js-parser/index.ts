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
import {ob1Get0} from "@internal/ob1";

export {default as CommentsConsumer} from "./CommentsConsumer";

export function parseJS(userOptions: JSParserUserOptions): JSRoot {
	const options: JSParserOptions = normalizeOptions(userOptions);
	return createJSParser(options).parse();
}

export function tokenizeJS(userOptions: JSParserUserOptions): Array<PublicToken> {
	const options: JSParserOptions = normalizeOptions(userOptions);
	const parser = createJSParser({...options, tokens: true});
	const root = parser.parse();

	const diagnostics = parser.getDiagnostics();
	let tokens: Array<PublicToken> = [];

	for (const token of parser.state.tokens) {
		tokens.push({
			type: token.type.label,
			start: parser.getIndexFromPosition(token.loc.start, token.loc.filename),
			end: parser.getIndexFromPosition(token.loc.end, token.loc.filename),
		});
	}

	// If we have any diagnostics, then mark anything from the first as invalid
	if (diagnostics.length > 0 && root.corrupt) {
		const firstDiag = diagnostics[0];
		const invalidStart = firstDiag.location.start;
		const invalidEnd = firstDiag.location.end;
		if (invalidStart === undefined || invalidEnd === undefined) {
			throw new Error("All parser diagnostics are expected to have a start/end");
		}

		const invalidToken: PublicToken = {
			type: "invalid",
			start: parser.getIndexFromPosition(
				invalidStart,
				firstDiag.location.filename,
			),
			end: parser.getIndexFromPosition(invalidEnd, firstDiag.location.filename),
		};

		// Remove all tokens after our invalid one
		tokens = tokens.filter((token) => {
			return ob1Get0(token.start) < ob1Get0(invalidToken.start);
		});

		tokens.push(invalidToken);
	}

	return tokens;
}

export {Token};

export {tokTypes};

export {keywords as keywordTokTypes} from "./tokenizer/types";
