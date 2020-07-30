/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceLocation} from "@internal/parser-core";

export type SpaceToken = {
	type: "Space";
};

export type LineToken = {
	type: "Line";
	mode: "space" | "soft" | "hard";
};

export type LineSuffixToken = {
	type: "LineSuffix";
	contents: Token;
};

export type IndentToken = {
	type: "Indent";
	contents: Token;
};

export type GroupToken = {
	type: "Group";
	contents: Token;
	shouldBreak: boolean;
};

export type CommentToken = {
	type: "Comment";
	value: Token;
};

export type PositionMarkerToken = {
	type: "PositionMarker";
	loc: SourceLocation;
	prop: "start" | "end";
};

export type ConcatToken = {
	type: "Concat";
	parts: Tokens;
};

export type IfBreakToken = {
	type: "IfBreak";
	breakContents: Token;
	flatContents: Token | undefined;
};

export type Tokens = Array<Token>;

export type Token =
	| string
	| CommentToken
	| ConcatToken
	| GroupToken
	| IfBreakToken
	| IndentToken
	| LineSuffixToken
	| LineToken
	| PositionMarkerToken
	| SpaceToken;

export const lineOrSpace: LineToken = {
	type: "Line",
	mode: "space",
};

export const softline: LineToken = {
	type: "Line",
	mode: "soft",
};

export const hardline: LineToken = {
	type: "Line",
	mode: "hard",
};

export const space: SpaceToken = {
	type: "Space",
};

export function group(contents: Token, shouldBreak: boolean = false): GroupToken {
	return {
		type: "Group",
		contents,
		shouldBreak,
	};
}

export function comment(value: Token): CommentToken {
	return {
		type: "Comment",
		value,
	};
}

export function indent(contents: Token, force: boolean = false): Token {
	if (contents === "") {
		return "";
	}
	if (force) {
		contents = concat([hardline, contents]);
	}
	return {
		type: "Indent",
		contents,
	};
}

export function mark(
	loc: SourceLocation,
	prop: "start" | "end",
): PositionMarkerToken {
	return {
		type: "PositionMarker",
		loc,
		prop,
	};
}

export function concat(parts: Tokens): Token {
	if (parts.length === 0) {
		return "";
	}

	if (parts.length === 1) {
		return parts[0];
	}

	return {
		type: "Concat",
		parts,
	};
}

export function ifBreak(
	breakContents: Token,
	flatContents?: Token,
): IfBreakToken {
	return {
		type: "IfBreak",
		breakContents,
		flatContents,
	};
}

export function join(separator: Token, tokens: Tokens): Token {
	if (tokens.length === 0) {
		return "";
	}

	if (tokens.length === 1) {
		return tokens[0];
	}

	const parts = [];

	for (let i = 0; i < tokens.length; i++) {
		if (i > 0) {
			parts.push(separator);
		}
		parts.push(tokens[i]);
	}

	return concat(parts);
}

export function lineSuffix(contents: Token): LineSuffixToken {
	return {
		type: "LineSuffix",
		contents,
	};
}
