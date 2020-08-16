/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, Number1, ob1Coerce1, ob1Number0Neg1} from "@internal/ob1";
import {AnyFilePath} from "@internal/path";
import {
	DiagnosticCategory,
	DiagnosticDescriptionOptional,
	DiagnosticLocation,
} from "@internal/diagnostics";
import {default as ParserCore} from "./ParserCore";
import {Dict} from "@internal/typescript-helpers";

// rome-ignore lint/ts/noExplicitAny
export type AnyParserCore = ParserCore<{
	tokens: TokensShape;
	state: Dict<any>;
	options: ParserOptions & Dict<any>;
	meta: Dict<any> | void;
}>;

export type ParserCoreTypes = {
	tokens: BaseTokens;
	state: object;
	options: ParserOptions;
	meta: object | void;
};

export type ParserCoreImplementation<Types extends ParserCoreTypes> = {
	diagnosticCategory: DiagnosticCategory;
	ignoreWhitespaceTokens?: boolean;
	retainCarriageReturn?: boolean;
	getInitialState?: (parser: ParserCore<Types>) => Types["state"];
	tokenize?: (
		parser: ParserCore<Types>,
		index: Number0,
	) => undefined | TokenValues<Types["tokens"]>;
	normalizeInput?: (input: string) => string;
	tokenizeWithState?: (
		parser: ParserCore<Types>,
		index: Number0,
		state: Types["state"],
	) => undefined | ParserCoreTokenizeState<Types>;
	overrides?: {
		getPosition: (parser: ParserCore<Types>) => Position;
		getIndex: (parser: ParserCore<Types>) => Number0;
		getLastEndPosition: (parser: ParserCore<Types>) => Position;
	};
};

export type ParserCoreTokenizeState<Types extends ParserCoreTypes> = {
	token: TokenValues<Types["tokens"]>;
	state: Types["state"];
};

//# Node types
export type NodeBase = {
	type: string;
	loc?: SourceLocation;
};

export type SimpleNode<Type extends string> = NodeBase & {
	type: Type;
};

export type ComplexNode<Type extends string, Data> = NodeBase &
	Data & {
		type: Type;
	};

export type ValueNode<Type extends string, Value> = NodeBase & {
	type: Type;
	value: Value;
};

//# Token types
export type TokenBase = {
	type: string;
	start: Number0;
	end: Number0;
};

export type TokensShape = {
	Invalid: InvalidToken;
	EOF: EOFToken;
	SOF: SOFToken;
	[type: string]: TokenBase;
};

export type SimpleToken<Type extends string> = TokenBase & {
	type: Type;
};

export type ComplexToken<Type extends string, Data> = TokenBase &
	Data & {
		type: Type;
	};

export type ValueToken<Type extends string, Value> = TokenBase & {
	type: Type;
	value: Value;
};

export type EOFToken = SimpleToken<"EOF">;

export type SOFToken = SimpleToken<"SOF">;

export type InvalidToken = SimpleToken<"Invalid">;

export type BaseTokens = {
	Invalid: InvalidToken;
	EOF: EOFToken;
	SOF: SOFToken;
};

//# Other types
export type SourceLocation = {
	filename?: string;
	identifierName?: string;
	start: Position;
	end: Position;
};

export type Position = {
	line: Number1;
	column: Number0;
};

export const UNKNOWN_POSITION: Position = {
	line: ob1Coerce1(-1),
	column: ob1Number0Neg1,
};

export type ParserOptions = {
	path?: string | AnyFilePath;
	mtime?: number;
	input?: string;
	sourceText?: string;
	offsetPosition?: Position;
};

export type ParserOptionsWithRequiredPath = Omit<ParserOptions, "path"> & {
	path: NonNullable<ParserOptions["path"]>;
};

export type ParserUnexpectedOptions = {
	description?: DiagnosticDescriptionOptional;
	loc?: SourceLocation;
	start?: Position;
	end?: Position;
	token?: TokenBase;
	index?: Number0;
	startIndex?: Number0;
	endIndex?: Number0;
	location?: DiagnosticLocation;
};

export type TokenValues<Tokens extends TokensShape> = TokenBase &
	(Tokens[keyof Tokens] | BaseTokens[keyof BaseTokens]);
