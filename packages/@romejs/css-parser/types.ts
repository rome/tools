import {
	BaseTokens,
	ComplexToken,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@romejs/parser-core";
import {DiagnosticCategory} from "@romejs/diagnostics";

export interface CSSParserOptions extends Omit<
	ParserOptions,
	"ignoreWhitespaceTokens"
> {
	consumeDiagnosticCategory?: DiagnosticCategory;
}

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
	Colon: SimpleToken<"Colon">;
	Comma: SimpleToken<"Comma">;
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
