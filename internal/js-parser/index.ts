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
import {Token} from "./tokenizer/index";
import {types as tokTypes} from "./tokenizer/types";
import {createJSParser} from "./parser";
import "./tokenizer/context";

export {default as CommentsConsumer} from "./CommentsConsumer";

export function parseJS(userOptions: JSParserUserOptions): JSRoot {
	const options: JSParserOptions = normalizeOptions(userOptions);
	return createJSParser(options).parse();
}

export function tokenizeJS(userOptions: JSParserUserOptions): Array<Token> {
	const options: JSParserOptions = normalizeOptions(userOptions);
	const parser = createJSParser({...options, tokens: true});
	const root = parser.parse();

	const diagnostics = parser.getDiagnostics();
	let tokens: Array<Token> = parser.state.tokens;

	// If we have any diagnostics, then mark anything from the first as invalid
	if (diagnostics.length > 0 && root.corrupt) {
		const firstDiag = diagnostics[0];
		const invalidStart = firstDiag.location.start;
		const invalidEnd = firstDiag.location.end;
		if (invalidStart === undefined || invalidEnd === undefined) {
			throw new Error("All parser diagnostics are expected to have a start/end");
		}

		const invalidStartIndex = invalidStart.index;

		const invalidToken: Token = {
			type: tokTypes.invalid,
			start: invalidStart.index,
			end: invalidEnd.index,
			loc: {
				filename: parser.filename,
				start: invalidStart,
				end: invalidEnd,
			},
		};

		// Remove all tokens after our invalid one
		tokens = tokens.filter((token) => {
			return token.loc.start.index >= invalidStartIndex;
		});

		tokens.push(invalidToken);
	}

	return tokens;
}

export {Token};

export {tokTypes};

export {keywords as keywordTokTypes} from "./tokenizer/types";
