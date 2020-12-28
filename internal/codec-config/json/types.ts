import {
	BaseTokens,
	NumberToken,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";

export type Tokens = BaseTokens & {
	BlockComment: StringToken<"BlockComment">;
	LineComment: StringToken<"LineComment">;
	String: StringToken<"String">;
	Number: NumberToken<"Number">;
	Word: StringToken<"Word">;
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
