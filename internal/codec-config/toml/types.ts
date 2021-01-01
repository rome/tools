import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";
import {JSONValue} from "../json/types";

export type Tokens = BaseTokens & {
	Text: StringToken<"Text">;
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
	String: StringToken<"String">;
	// .
	Dot: SimpleToken<"Dot">;
	// ,
	Comma: SimpleToken<"Comma">;
	// #
	Hash: SimpleToken<"Hash">;
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
