import {BaseTokens, SimpleToken, ValueToken} from "@internal/parser-core";

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

export type JSONArray = JSONValue[];
