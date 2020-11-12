import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@internal/parser-core";
import {JSONValue} from "../json/types";

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
	// "VALUE"
	// 'VALUE'
	String: ValueToken<"String", string>;
	// .
	Dot: SimpleToken<"Dot">;
	// ,
	Comma: SimpleToken<"Comma">;
	// #
	Hash: SimpleToken<"Hash">;
	Text: ValueToken<"Text", string>;
};

export type TOMLParserTypes = {
	tokens: Tokens;
	options: ParserOptions;
	state: State;
	meta: void;
};

export type State = {};

export type TOMLParser = ParserCore<TOMLParserTypes>;

export type TOMLValue = JSONValue;
