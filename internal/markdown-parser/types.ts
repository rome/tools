import {
	BaseTokens,
	ComplexToken,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@internal/parser-core";
import {DiagnosticCategory} from "@internal/diagnostics";

export interface MarkdownParserOptions extends Omit<
	ParserOptions,
	"ignoreWhitespaceTokens"
> {
	consumeDiagnosticCategory?: DiagnosticCategory;
}

export type State = {
	isBlockHead: boolean;
};

export type ListProperties = {
	checked: boolean | undefined;
	numeric: boolean;
	value?: string;
};

export type Tokens = BaseTokens & {
	HeadingLevel: ValueToken<"HeadingLevel", number>;
	Greater: SimpleToken<"Greater">;
	Text: ValueToken<"Text", string>;
	NewLine: SimpleToken<"NewLine">;
	Break: ValueToken<"Break", string>;
	ListItem: ComplexToken<"ListItem", ListProperties>;
	EndParagraph: SimpleToken<"EndParagraph">;
};
