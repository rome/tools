/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ScopeType} from "../parser";
import {Position, SourceLocation} from "@internal/parser-core";
import {TokContext} from "./context";
import {TokenTypes} from "./types";
import {Token} from "..";

type Scopes = {[K in ScopeType]?: unknown[]};

export type State = {
	isIterator: boolean;
	tokens: Token[];
	hasHoistedVars: boolean;
	indentLevel: number;
	lineStart: boolean;

	// Used to signify the start of a potential arrow function
	potentialArrowAt: number;

	// Used to signify the start of an expression which looks like a
	// typed arrow function, but it isn't
	// e.g. a ? (b) : c => d
	//          ^
	noArrowAt: number[];

	// Used to signify the start of an expression whose params, if it looks like
	// an arrow function, shouldn't be converted to assignable nodes.
	// This is used to defer the validation of typed arrow functions inside
	// conditional expressions.
	// e.g. a ? (b) : c => d
	//          ^
	noArrowParamsConversionAt: number[];

	// Flags to track whether we are in a function, a generator.
	maybeInArrowParameters: boolean;
	noAnonFunctionType: boolean;

	// A comma after "...a" is only allowed in spread, but not in rest.
	// Since we parse destructuring patterns as array/object literals
	// and then convert them, we need to track it.
	commaAfterSpreadAt: number;

	// Positions to delayed-check that yield/await does not exist in default parameters.
	yieldPos: number;
	awaitPos: number;

	// Check whether we are in a (nested) class or not.
	classLevel: number;

	// Labels in scope.
	labels: Label[];

	// The first yield expression inside parenthesized expressions and arrow
	// function parameters. It is used to disallow yield in arrow function
	// parameters.
	yieldInPossibleArrowParameters: undefined | Position;

	// The current position of the tokenizer in the input.
	index: number;
	lineStartIndex: number;
	curLine: number;

	// Token type
	tokenType: TokenTypes;

	// For tokens that include more information than their type, the value
	tokenValue: unknown;

	// Current token offsets
	startPos: Position;
	startIndex: number;
	endPos: Position;
	endIndex: number;

	// Position information for the previous token
	lastEndPos: Position;
	lastEndIndex: number;
	lastStartPos: Position;
	lastStartIndex: number;

	// The context stack is used to superficially track syntactic
	// context to predict whether a regular expression is allowed in a
	// given position.
	context: TokContext[];
	exprAllowed: boolean;

	// Used to signal to callers of `readWord1` whether the word
	// contained any escape sequences. This is needed because words with
	// escape sequences must not be interpreted as keywords.
	escapePosition: undefined | number;

	//
	containsOctal: boolean;
	octalPosition: undefined | number;

	// Names of exports store. `default` is stored as a name for both
	// `export default foo;` and `export { foo as default };`.
	exportedIdentifiers: Map<string, SourceLocation>;
	invalidTemplateEscapePosition: undefined | number;
	scopes: Scopes;
};

export type LabelKind = undefined | "loop" | "switch";

export type Label = {
	kind: LabelKind;
	loc?: SourceLocation;
	name?: string;
	statementStart?: number;
};
