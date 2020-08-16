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
import {Number0, Number1} from "@internal/ob1";

type Scopes = {[K in ScopeType]?: Array<unknown>};

export type State = {
	isIterator: boolean;
	tokens: Array<Token>;
	hasHoistedVars: boolean;
	indentLevel: Number0;
	lineStart: boolean;

	// Used to signify the start of a potential arrow function
	potentialArrowAt: Number0;

	// Used to signify the start of an expression which looks like a
	// typed arrow function, but it isn't
	// e.g. a ? (b) : c => d
	//          ^
	noArrowAt: Array<Number0>;

	// Used to signify the start of an expression whose params, if it looks like
	// an arrow function, shouldn't be converted to assignable nodes.
	// This is used to defer the validation of typed arrow functions inside
	// conditional expressions.
	// e.g. a ? (b) : c => d
	//          ^
	noArrowParamsConversionAt: Array<Number0>;

	// Flags to track whether we are in a function, a generator.
	maybeInArrowParameters: boolean;
	noAnonFunctionType: boolean;

	// A comma after "...a" is only allowed in spread, but not in rest.
	// Since we parse destructuring patterns as array/object literals
	// and then convert them, we need to track it.
	commaAfterSpreadAt: Number0;

	// Positions to delayed-check that yield/await does not exist in default parameters.
	yieldPos: Number0;
	awaitPos: Number0;

	// Check whether we are in a (nested) class or not.
	classLevel: Number0;

	// Labels in scope.
	labels: Array<Label>;

	// The first yield expression inside parenthesized expressions and arrow
	// function parameters. It is used to disallow yield in arrow function
	// parameters.
	yieldInPossibleArrowParameters: undefined | Position;

	// The current position of the tokenizer in the input.
	index: Number0;
	lineStartIndex: Number0;
	curLine: Number1;

	// Token type
	tokenType: TokenTypes;

	// For tokens that include more information than their type, the value
	tokenValue: unknown;

	// Current token offsets
	startPos: Position;
	startIndex: Number0;
	endPos: Position;
	endIndex: Number0;

	// Position information for the previous token
	lastEndPos: Position;
	lastEndIndex: Number0;
	lastStartPos: Position;
	lastStartIndex: Number0;

	// The context stack is used to superficially track syntactic
	// context to predict whether a regular expression is allowed in a
	// given position.
	context: Array<TokContext>;
	exprAllowed: boolean;

	// Used to signal to callers of `readWord1` whether the word
	// contained any escape sequences. This is needed because words with
	// escape sequences must not be interpreted as keywords.
	escapePosition: undefined | Number0;

	//
	containsOctal: boolean;
	octalPosition: undefined | Number0;

	// Names of exports store. `default` is stored as a name for both
	// `export default foo;` and `export { foo as default };`.
	exportedIdentifiers: Map<string, SourceLocation>;
	invalidTemplateEscapePosition: undefined | Number0;
	scopes: Scopes;
};

export type LabelKind = undefined | "loop" | "switch";

export type Label = {
	kind: LabelKind;
	loc?: SourceLocation;
	name?: string;
	statementStart?: Number0;
};
