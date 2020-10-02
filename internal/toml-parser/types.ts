import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@internal/parser-core";

export type Tokens = BaseTokens & {
	// [
	OpenSquareBracket: SimpleToken<"OpenSquareBracket">;
	// ]
	CloseSquareBracket: SimpleToken<"CloseSquareBracket">;
	// {
	OpenCurlyBracket: SimpleToken<"OpenCurlyBracket">;
	// }
	CloseCurlyBracket: SimpleToken<"CloseCurlyBracket">;
	// =
	Equals: SimpleToken<"Equals">;
	// "
	Quote: SimpleToken<"Quote">;
	// .
	Dot: SimpleToken<"Dot">;
	// ,
	Comma: SimpleToken<"Comma">;
	// #
	Hash: SimpleToken<"Hash">;
	Text: ValueToken<"Text", string>;
};

export type TomlParserTypes = {
	tokens: Tokens;
	options: ParserOptions;
	state: {};
	meta: void;
};

export type TomlParser = ParserCore<TomlParserTypes>;
