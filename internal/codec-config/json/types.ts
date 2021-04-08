import {
	BaseTokens,
	NumberToken,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";

export type Tokens = BaseTokens & {
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

// JSON type that includes toJSON

export type ToJSONValue =
	| null
	| string
	| number
	| boolean
	| ToJSONObject
	| ToJSONArray
	| ToJSONObjectMethod;

export type ToJSONPropertyValue = undefined | void | ToJSONValue;

export type ToJSONObject = {
	[x: string]: ToJSONPropertyValue;
};

export type ToJSONArray = ToJSONValue[];

export type ToJSONObjectMethod = {
	toJSON: () => JSONValue;
};
