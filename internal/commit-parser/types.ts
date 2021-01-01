import {BaseTokens, SimpleToken, StringToken} from "@internal/parser-core";

export const Symbols = {
	Space: " ",
	Tab: "\t",
};

export type Tokens = BaseTokens & {
	Colon: SimpleToken<"Colon">;
	Exclamation: SimpleToken<"Exclamation">;
	LeftParen: SimpleToken<"LeftParen">;
	RightParen: SimpleToken<"RightParen">;
	Whitespace: SimpleToken<"Whitespace">;
	Word: StringToken<"Word">;
};
