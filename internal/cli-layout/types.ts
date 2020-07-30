import {
	MarkupFormatOptions,
	MarkupLineWrapMode,
	MarkupParsedChildren,
} from "@internal/markup";
import {TerminalFeatures} from "@internal/cli-environment";
import {Number1} from "@internal/ob1";

export type GridPointer = {
	char: MarkupParsedChildren;
	message: MarkupParsedChildren;
	line: Number1;
	columnStart: Number1;
	columnEnd: Number1;
};

export type UserGridOptions = MarkupFormatOptions & {
	convertTabs?: boolean;
	features?: TerminalFeatures;
	columns?: Number1;
};

export type GridViewOptions = {
	extraSoftWrapIndent?: number;
	lineWrapMode?: MarkupLineWrapMode;
	pointer?: GridPointer;
};

export type GridOptions = UserGridOptions & {
	sourceText: string;
	view: GridViewOptions;
};

export type GridOutputFormat = "ansi" | "html" | "none";
