import {
	BaseTokens,
	ComplexToken,
	ParserOptions,
	SimpleToken,
	ValueToken,
} from "@internal/parser-core";
import {DiagnosticCategory} from "@internal/diagnostics";
import {InlineState} from "@internal/markdown-parser/State";
import {Number0} from "@internal/ob1";

export interface MarkdownParserOptions extends Omit<
	ParserOptions,
	"ignoreWhitespaceTokens"
> {
	consumeDiagnosticCategory?: DiagnosticCategory;
}

export type MarkdownParserState = {
	isBlockHead: boolean;
	isParagraph: boolean;
	inlineState: InlineState;
	isListItem: boolean;
};

export type ListProperties = {
	checked: boolean | undefined;
	numeric: boolean;
	/**
	 * In case of a numeric list item
	 */
	value?: "*" | "-";
};

export type CodeProperties = {
	language: string;
};

/**
 * This type is used to determine if the character is used for inline styling or not
 * https://github.github.com/gfm/#emphasis-and-strong-emphasis
 */
export type DelimiterRun = {
	// the index of the counter part that closes the delimiter
	closingIndexOfDelimiter?: Number0;
	leftFlankingDelimiter?: boolean;
	rightFlankingDelimiter?: boolean;
	value: string;
};

export type Emphasis = ComplexToken<"Emphasis", DelimiterRun>;
export type Strong = ComplexToken<"Strong", DelimiterRun>;

export type Tokens = BaseTokens & {
	HeadingLevel: ValueToken<"HeadingLevel", number>;
	Greater: SimpleToken<"Greater">;
	Text: ValueToken<"Text", string>;
	NewLine: SimpleToken<"NewLine">;
	Break: ValueToken<"Break", string>;
	ListItem: ComplexToken<"ListItem", ListProperties>;
	Code: ComplexToken<"Code", CodeProperties>;
	// [
	OpenSquareBracket: SimpleToken<"OpenSquareBracket">;
	// ]
	CloseSquareBracket: SimpleToken<"CloseSquareBracket">;
	// (
	OpenBracket: SimpleToken<"OpenBracket">;
	// )
	CloseBracket: SimpleToken<"CloseBracket">;
	// for * and _
	Emphasis: Emphasis;
	// for ** and __
	Strong: Strong;
	TablePipe: SimpleToken<"TablePipe">;
};
