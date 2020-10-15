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
	NewLine: SimpleToken<"NewLine">;
	Space: SimpleToken<"Space">;
	// "
	DoubleQuote: SimpleToken<"DoubleQuote">;
	// '
	SingleQuote: SimpleToken<"SingleQuote">;
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
	state: State;
	meta: void;
};

export type State = {
	/**
	 * Reading the value
	 */
	inValue: undefined | "'" | '"';
};

export type TomlParser = ParserCore<TomlParserTypes>;
