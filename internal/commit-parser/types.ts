import {
	BaseTokens,
	SimpleToken,
	StringToken,
} from "@internal/parser-core";
import {Diagnostic} from "@internal/diagnostics";

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

export interface ConventionalCommit {
	readonly diagnostics: Diagnostic[];
	readonly breaking: boolean;
	readonly commitType: string;
	readonly custom: boolean;
	readonly rawBody: string;
	readonly scope: string;
}
