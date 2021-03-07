import {ZeroIndexed} from "@internal/numbers";
import {DiagnosticLanguage, DiagnosticSourceType} from "@internal/diagnostics";
import {Path} from "@internal/path";
import {MarkupTokenType, StaticMarkup} from "@internal/markup";

export type AnsiHighlightOptions = {
	path: Path;
	input: string;
	sourceTypeJS: undefined | DiagnosticSourceType;
	language: DiagnosticLanguage;
	highlight: boolean;
};

export type TokenShape = {
	start: ZeroIndexed;
	end: ZeroIndexed;
};

export type ReduceCallbackResult = {
	type?: MarkupTokenType;
	value?: StaticMarkup;
};

export type ReduceCallback<Token extends TokenShape> = (
	token: Token,
	line: StaticMarkup,
	prev: undefined | Token,
	next: undefined | Token,
) => undefined | ReduceCallbackResult;

export type HighlightCodeResult = StaticMarkup[];
