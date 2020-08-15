/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BaseTokens, SimpleToken, ValueToken} from "@internal/parser-core";
import {AbsoluteFilePath} from "@internal/path";
import {UserConfig} from "@internal/core";
import {Number0, Number1} from "@internal/ob1";
import {Consumer} from "@internal/consume";

export type Tokens = BaseTokens & {
	Text: ValueToken<"Text", string>;
	Slash: SimpleToken<"Slash">;
	Less: SimpleToken<"Less">;
	Equals: SimpleToken<"Equals">;
	Greater: SimpleToken<"Greater">;
	Word: ValueToken<"Word", string>;
	String: ValueToken<"String", string>;
};

export type MarkupParsedText = {
	type: "Text";
	source: boolean;
	sourceValue?: string;
	value: string;
};

export type MarkupParsedAttributes = Consumer;

export type MarkupParsedTag = {
	type: "Tag";
	name: MarkupTagName;
	attributes: MarkupParsedAttributes;
	children: MarkupParsedChildren;
};

export type MarkupParsedChild = MarkupParsedText | MarkupParsedTag;

export type MarkupParsedChildren = Array<MarkupParsedChild>;

export type MarkupLineWrapMode = "none" | "char-break" | "word-break";

export type MarkupTagName =
	| "indent"
	| "view"
	| "viewLinePrefix"
	| "viewPointer"
	| "token"
	| "hr"
	| "pad"
	| "grammarNumber"
	| "code"
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

export type MarkupFormatNormalizeOptions = MarkupFormatOptions & {
	stripPositions?: boolean;
	stripFilelinkText?: boolean;
};

export type MarkupLinesAndWidth = {
	width: number;
	lines: Array<string>;
};

export type MarkupRGB = [number, number, number];

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
	| "boolean"
	| "important";

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
