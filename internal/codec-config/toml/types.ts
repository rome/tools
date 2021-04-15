import {ZeroIndexed} from "@internal/numbers";
import {
	BaseTokens,
	ComplexToken,
	ParserCore,
	ParserOptions,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";
import {PathComments} from "../types";

export type Tokens = BaseTokens & {
	Word: StringToken<"Word">;
	Int: StringToken<"Int">;
	Float: StringToken<"Float">;
	Comment: StringToken<"Comment">;
	Date: ComplexToken<
		"Date",
		{
			year: number;
			month: number;
			day: number;
		}
	>;
	Time: ComplexToken<
		"Time",
		{
			hours: number;
			minutes: number;
			seconds: number;
		}
	>;
	DateTime: ComplexToken<
		"DateTime",
		{
			year: number;
			month: number;
			day: number;
			hours: number;
			minutes: number;
			seconds: number;
			utc: boolean;
			offset?: {
				negative: boolean;
				hours: number;
				minutes: number;
			};
		}
	>;
	// [
	OpenSquareBracket: SimpleToken<"OpenSquareBracket">;
	// ]
	CloseSquareBracket: SimpleToken<"CloseSquareBracket">;
	// {
	OpenCurlyBrace: SimpleToken<"OpenCurlyBrace">;
	// }
	CloseCurlyBrace: SimpleToken<"CloseCurlyBrace">;
	// :
	Colon: SimpleToken<"Colon">;
	// =
	Equals: SimpleToken<"Equals">;
	// "VALUE"
	// 'VALUE'
	// """VALUE"""
	// '''VALUE'''
	String: StringToken<"String">;
	// .
	Dot: SimpleToken<"Dot">;
	// ,
	Comma: SimpleToken<"Comma">;
	// #
	Hash: SimpleToken<"Hash">;
	// +
	Plus: SimpleToken<"Plus">;
	// -
	Minus: SimpleToken<"Minus">;
};

export type TOMLParserTypes = {
	tokens: Tokens;
	options: ParserOptions;
	state: State;
	meta: void;
};

export type TOMLKeys = TOMLKey[];

export type TOMLKey = {
	key: string;
	start?: ZeroIndexed;
	end?: ZeroIndexed;
};

export type State = {
	explicitDefinedPaths: Set<string>;
	pathComments: Map<string, PathComments>;
};

export type TOMLParser = ParserCore<TOMLParserTypes>;

export type TOMLValue =
	| null
	| string
	| number
	| boolean
	| Date
	| TOMLObject
	| TOMLArray;

export type TOMLObject = {
	[x: string]: TOMLValue;
};

export type TOMLArray = TOMLValue[];
