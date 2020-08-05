import {
	BaseTokens,
	ComplexToken,
	ParserOptionsWithRequiredPath,
	SimpleToken,
	ValueToken,
} from "@internal/parser-core";
import {
	CSSBlock,
	CSSDimension,
	CSSFunction,
	CSSIdentifier,
	CSSNumber,
	CSSPercentage,
	CSSRaw,
} from "@internal/ast";

export interface CSSParserOptions extends Omit<
	ParserOptionsWithRequiredPath,
	"ignoreWhitespaceTokens"
> {}

export interface DimensionData {
	numberType: string;
	unit: string;
	value: number;
}

export interface HashData {
	hashType?: string;
	value: string;
}

export interface NumberData {
	numberType: string;
	value: number;
}

export type Tokens = BaseTokens & {
	AtKeyword: ValueToken<"AtKeyword", string>;
	BadString: SimpleToken<"BadString">;
	BadURL: SimpleToken<"BadURL">;
	CDC: SimpleToken<"CDC">;
	CDO: SimpleToken<"CDO">;
	Colon: SimpleToken<"Colon">;
	Comma: SimpleToken<"Comma">;
	Comment: ValueToken<"Comment", string>;
	Delim: ValueToken<"Delim", string>;
	Dimension: ComplexToken<"Dimension", DimensionData>;
	Function: ValueToken<"Function", string>;
	Hash: ComplexToken<"Hash", HashData>;
	Ident: ValueToken<"Ident", string>;
	LeftCurlyBracket: SimpleToken<"LeftCurlyBracket">;
	LeftParen: SimpleToken<"LeftParen">;
	LeftSquareBracket: SimpleToken<"LeftSquareBracket">;
	Number: ComplexToken<"Number", NumberData>;
	Percentage: ValueToken<"Percentage", number>;
	RightCurlyBracket: SimpleToken<"RightCurlyBracket">;
	RightParen: SimpleToken<"RightParen">;
	RightSquareBracket: SimpleToken<"RightSquareBracket">;
	Semi: SimpleToken<"Semi">;
	String: ValueToken<"String", string>;
	URL: ValueToken<"URL", string>;
	Whitespace: SimpleToken<"Whitespace">;
};

export type AnyCSSToken = Tokens[keyof Tokens];

export type AnyCSSValue =
	| CSSFunction
	| CSSBlock
	| CSSDimension
	| CSSPercentage
	| CSSIdentifier
	| CSSNumber
	| CSSRaw;
