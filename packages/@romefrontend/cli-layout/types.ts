/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BaseTokens, SimpleToken, ValueToken} from "@romefrontend/parser-core";
import {AbsoluteFilePath} from "@romefrontend/path";
import {UserConfig} from "@romefrontend/core/common/userConfig";
import {Number0, Number1} from "@romefrontend/ob1";
import {TerminalFeatures} from "@romefrontend/environment";
import {Consumer} from "@romefrontend/consume";

export type Tokens = BaseTokens & {
	Text: ValueToken<"Text", string>;
	Slash: SimpleToken<"Slash">;
	Less: SimpleToken<"Less">;
	Equals: SimpleToken<"Equals">;
	Greater: SimpleToken<"Greater">;
	Word: ValueToken<"Word", string>;
	String: ValueToken<"String", string>;
};

//
export type TextNode = {
	type: "Text";
	source: boolean;
	sourceValue?: string;
	value: string;
};

export type TagAttributes = Consumer;

export type TagNode = {
	type: "Tag";
	name: MarkupTagName;
	attributes: TagAttributes;
	children: Children;
};

export type MarkupLineWrapMode = "none" | "char-break" | "word-break";

export type ChildNode = TextNode | TagNode;

export type Children = Array<ChildNode>;

export type MarkupTagName =
	| "indent"
	| "view"
	| "viewLinePrefix"
	| "viewPointer"
	| "token"
	| "hr"
	| "pad"
	| "grammarNumber"
	| "command"
	| "inverse"
	| "dim"
	| "emphasis"
	| "number"
	| "hyperlink"
	| "filelink"
	| "duration"
	| "filesize"
	| "italic"
	| "underline"
	| "strike"
	| "error"
	| "success"
	| "warn"
	| "info"
	| "highlight"
	| "color"
	| "table"
	| "tr"
	| "td"
	| "nobr"
	| "ol"
	| "ul"
	| "li";

export type MarkupFormatPositionNormalizer = (
	filename: string,
	line: undefined | Number1,
	column: undefined | Number0,
) => {
	filename: string;
	line?: Number1;
	column?: Number0;
};

export type MarkupFormatFilenameHumanizer = (
	filename: string,
) => undefined | string;

export type MarkupFormatOptions = {
	userConfig?: UserConfig;
	normalizePosition?: MarkupFormatPositionNormalizer;
	humanizeFilename?: MarkupFormatFilenameHumanizer;
	cwd?: AbsoluteFilePath;
};

export type MarkupPointer = {
	char: Children;
	message: Children;
	line: Number1;
	columnStart: Number1;
	columnEnd: Number1;
};

export type UserMarkupFormatGridOptions = MarkupFormatOptions & {
	features?: TerminalFeatures;
	columns?: Number1;
};

export type MarkupGridViewOptions = {
	extraSoftWrapIndent?: number;
	lineWrapMode?: MarkupLineWrapMode;
	pointer?: MarkupPointer;
};

export type MarkupFormatGridOptions = UserMarkupFormatGridOptions & {
	sourceText: string;
	view: MarkupGridViewOptions;
};

export type MarkupFormatNormalizeOptions = MarkupFormatOptions & {
	stripPositions?: boolean;
};

export type MarkupLinesAndWidth = {
	width: number;
	lines: Array<string>;
};

export type GridOutputFormat = "ansi" | "html" | "none";

// These match PrismJS class names
export type MarkupTokenType =
	| "keyword"
	| "number"
	| "regex"
	| "string"
	| "comment"
	| "operator"
	| "punctuation"
	| "variable"
	| "attr-name"
	| "attr-value"
	| "attr-equals"
	| "tag"
	| "function"
	| "boolean";

export type MarkupColor =
	| "black"
	| "brightBlack"
	| "red"
	| "brightRed"
	| "green"
	| "brightGreen"
	| "yellow"
	| "brightYellow"
	| "blue"
	| "brightBlue"
	| "magenta"
	| "brightMagenta"
	| "cyan"
	| "brightCyan"
	| "white"
	| "brightWhite";
