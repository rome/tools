import {
	MarkupFormatOptions,
	MarkupLineWrapMode,
	MarkupParsedChildren,
} from "@internal/markup";
import {TerminalFeatures} from "@internal/cli-environment";
import {OneIndexed} from "@internal/numbers";
import {Position} from "@internal/parser-core";

export type GridPointer = {
	char: MarkupParsedChildren;
	message: MarkupParsedChildren;
	line: OneIndexed;
	columnStart: OneIndexed;
	columnEnd: OneIndexed;
};

export type UserGridOptions = MarkupFormatOptions & {
	convertTabs?: boolean;
	features?: TerminalFeatures;
	columns?: OneIndexed;
};

export type GridViewOptions = {
	extraSoftWrapIndent?: number;
	lineWrapMode?: MarkupLineWrapMode;
	pointer?: GridPointer;
};

export type GridOptions = UserGridOptions & {
	throwOnNewline?: boolean;
	sourceText: string;
	view: GridViewOptions;
};

export type GridOutputFormat = "ansi" | "html" | "none";

export type GridLocator = {
	start: Position;
	end: Position;
};

export type GridLocators = Map<string, GridLocator>;
