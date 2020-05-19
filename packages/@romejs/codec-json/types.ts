/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@romejs/parser-core";
import {ConsumeContext} from "@romejs/consume";
import {DiagnosticCategory} from "@romejs/diagnostics";

export type JSONParserOptions = Omit<ParserOptions, "retainCarriageReturn"> & {
	consumeDiagnosticCategory?: DiagnosticCategory;
};

export type PathComments = {
	inner: Comments;
	outer: Comments;
};

export type PathToComments = Map<string, PathComments>;

export type LineComment = {
	type: "LineComment";
	value: string;
};

export type BlockComment = {
	type: "BlockComment";
	value: string;
};

export type Comments = Array<BlockComment | LineComment>;

export type JSONParserResult = {
	value: JSONValue;
	context: Required<ConsumeContext>;
};

export type Tokens = BaseTokens & {
	BlockComment: ValueToken<"BlockComment", string>;
	LineComment: ValueToken<"LineComment", string>;
	String: ValueToken<"String", string>;
	Number: ValueToken<"Number", number>;
	Word: ValueToken<"Word", string>;
	BracketOpen: SimpleToken<"BracketOpen">;
	BracketClose: SimpleToken<"BracketClose">;
	BraceOpen: SimpleToken<"BraceOpen">;
	BraceClose: SimpleToken<"BraceClose">;
	Comma: SimpleToken<"Comma">;
	Colon: SimpleToken<"Colon">;
	Dot: SimpleToken<"Dot">;
	Minus: SimpleToken<"Minus">;
	Plus: SimpleToken<"Plus">;
};

//
export type JSONValue =
	| null
	| string
	| number
	| boolean
	| JSONObject
	| JSONArray;

export type JSONPropertyValue = undefined | void | JSONValue;

export type JSONObject = {
	[x: string]: JSONPropertyValue;
};

export type JSONArray = Array<JSONValue>;
