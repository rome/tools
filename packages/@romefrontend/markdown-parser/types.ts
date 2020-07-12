import {
	BaseTokens,
	ParserCoreState,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@romefrontend/parser-core";
import {DiagnosticCategory} from "@romefrontend/diagnostics";

export interface MarkdownParserOptions extends Omit<
	ParserOptions,
	"ignoreWhitespaceTokens"
> {
	consumeDiagnosticCategory?: DiagnosticCategory;
}

export type State = ParserCoreState & {
	isBlockHead: boolean;
};

export type Tokens = BaseTokens & {
	HeadingLevel: ValueToken<"HeadingLevel", number>;
	Greater: SimpleToken<"Greater">;
	Text: ValueToken<"Text", string>;
	NewLine: SimpleToken<"NewLine">;
	Break: SimpleToken<"Break">;
	ListItem: ValueToken<"ListItem", string>;
};
