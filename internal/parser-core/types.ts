/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {IndexedNumberish, OneIndexed, ZeroIndexed} from "@internal/numbers";
import {Path, Pathish} from "@internal/path";
import {
	DiagnosticCategory,
	DiagnosticDescriptionOptional,
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticTags,
} from "@internal/diagnostics";
import {default as ParserCore} from "./ParserCore";
import {Dict} from "@internal/typescript-helpers";
import {AnyNode} from "@internal/ast";
import TokenizerCore from "./TokenizerCore";

export type ParserCoreReadCallback = (
	char: string,
	index: ZeroIndexed,
	input: string,
) => boolean;

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type AnyParserCore = ParserCore<{
	tokens: TokensShape;
	state: Dict<any>;
	options: ParserOptions & Dict<any>;
	meta: Dict<any> | void;
}>;

export type ParserCoreFactory<Types extends ParserCoreTypes> = {
	create(
		opts: Types["options"],
		meta: Types["meta"],
		overrides?: ParserCoreOverrides,
	): ParserCore<Types>;
};

export type ParserCoreTypes = {
	tokens: BaseTokens;
	state: object;
	options: ParserOptions;
	meta: object | void;
};

export type ParserCoreOverrides = {
	diagnosticCategory?: DiagnosticCategory;
	diagnosticLanguage?: DiagnosticLanguage;
	diagnosticCategoryValue?: string;
};

export type ParserCoreImplementation<Types extends ParserCoreTypes> = {
	diagnosticLanguage: DiagnosticLanguage;
	diagnosticCategory?: DiagnosticCategory;
	diagnosticTags?: DiagnosticTags;
	diagnosticCategoryValue?: string;
	caseInsensitiveTokenMatches?: boolean;
	ignoreWhitespaceTokens?: boolean;
	retainCarriageReturn?: boolean;
	getInitialState?: (parser: ParserCore<Types>) => Types["state"];
	tokenize?: (
		parser: ParserCore<Types>,
		tokenizer: TokenizerCore<Types>,
	) => undefined | TokenValues<Types["tokens"]>;
	normalizeInput?: (input: string) => string;
	tokenizeWithState?: (
		parser: ParserCore<Types>,
		tokenizer: TokenizerCore<Types>,
		state: Types["state"],
	) => undefined | ParserCoreTokenizeState<Types>;
	overrides?: {
		getPosition: (parser: ParserCore<Types>) => Position;
		getIndex: (parser: ParserCore<Types>) => ZeroIndexed;
		getLastEndPosition: (parser: ParserCore<Types>) => Position;
	};
	parseTemplate?: (opts: ParserOptions) => unknown;
};

export type ParserCoreTokenizeState<Types extends ParserCoreTypes> =
	| TokenValues<Types["tokens"]>
	| [Partial<Types["state"]>, TokenValues<Types["tokens"]>];

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
	start: ZeroIndexed;
	end: ZeroIndexed;
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

export type StringToken<Type extends string> = ValueToken<Type, string>;

export type NumberToken<Type extends string> = ValueToken<Type, number>;

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
	path: Path;
	identifierName?: string;
	start: Position;
	end: Position;
};

export type SourceLocationish = {
	path: Pathish;
	identifierName?: string;
	start: Positionish;
	end: Positionish;
};

export type Position = {
	line: OneIndexed;
	column: ZeroIndexed;
};

export type Positionish = {
	line: IndexedNumberish;
	column: IndexedNumberish;
};

export type PositionLike = {
	line?: undefined | OneIndexed;
	column?: undefined | ZeroIndexed;
};

export const UNKNOWN_POSITION: Position = {
	line: new OneIndexed(-1),
	column: new ZeroIndexed(-1),
};

export type ParserOptions = {
	path?: Path;
	integrity?: DiagnosticIntegrity;
	input?: string;
	sourceText?: string;
	offsetPosition?: Position;
	includeSourceTextInDiagnostics?: boolean;
};

export type ParserUnexpectedOptions = {
	description?: DiagnosticDescriptionOptional;
	node?: AnyNode;
	loc?: SourceLocation;
	start?: Position;
	end?: Position;
	token?: TokenBase;
	index?: number | ZeroIndexed;
	startIndex?: number | ZeroIndexed;
	endIndex?: number | ZeroIndexed;
};

export type TokenValues<Tokens extends TokensShape> = TokenBase &
	(Tokens[keyof Tokens] | BaseTokens[keyof BaseTokens]);
