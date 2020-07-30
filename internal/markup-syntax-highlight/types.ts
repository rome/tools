import {Number0} from "@internal/ob1";
import {DiagnosticLanguage, DiagnosticSourceType} from "@internal/diagnostics";
import {UnknownFilePath} from "@internal/path";
import {Markup, MarkupTokenType} from "@internal/markup";

export type AnsiHighlightOptions = {
	path: UnknownFilePath;
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
	value?: Markup;
};

export type ReduceCallback<Token extends TokenShape> = (
	token: Token,
	line: Markup,
	prev: undefined | Token,
	next: undefined | Token,
) => undefined | ReduceCallbackResult;

export type HighlightCodeResult = Array<Markup>;
