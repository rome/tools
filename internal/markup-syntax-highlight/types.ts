import {Number0} from "@internal/ob1";
import {DiagnosticLanguage, DiagnosticSourceType} from "@internal/diagnostics";
import {UnknownPath} from "@internal/path";
import {MarkupTokenType, StaticMarkup} from "@internal/markup";
import {AnyMarkups} from "@internal/markup/escape";

export type AnsiHighlightOptions = {
	path: UnknownPath;
	input: string;
	sourceTypeJS: undefined | DiagnosticSourceType;
	language: DiagnosticLanguage;
	highlight: boolean;
};

export type TokenShape = {
	start: Number0;
	end: Number0;
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

export type HighlightCodeResult = AnyMarkups;
