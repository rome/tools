import {
	BaseTokens,
	NodeBase,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";

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

export interface Commit extends NodeBase {
	readonly type: "Commit";
	readonly breaking: boolean;
	readonly commitType: string;
	readonly custom: boolean;
	readonly rawBody: string;
	readonly scope: string;
}
