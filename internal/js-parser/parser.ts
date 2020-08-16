/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ConstJSProgramSyntax,
	JSIdentifier,
	JSRoot,
} from "@internal/ast";
import {
	BaseTokens,
	ParserCore,
	ParserCoreState,
	ParserUnexpectedOptions,
	Position,
	comparePositions,
	createParser,
	derivePositionKey,
} from "@internal/parser-core";
import {JSParserOptions} from "./options";
import {DiagnosticDescription, descriptions} from "@internal/diagnostics";
import ParserBranchFinder from "./ParserBranchFinder";
import {Token, nextToken} from "./tokenizer/index";
import {TokenType, types as tt} from "./tokenizer/types";
import {lineBreak} from "@internal/js-parser-utils";
import {parseTopLevel} from "./parser/index";
import {State} from "./tokenizer/state";
import {
	Number0,
	ob1Number0,
	ob1Number0Neg1,
	ob1Number1,
	ob1Sub,
} from "@internal/ob1";
import {Dict, OptionalProps} from "@internal/typescript-helpers";
import {types as ct} from "@internal/js-parser/tokenizer/context";

const TOKEN_MISTAKES: Dict<string> = {
	";": ":",
	",": ".",
};

export class DiagnosticsFatalError extends Error {
	constructor() {
		super(
			"Diagnostics exceeded maxDiagnostics state cap, this error is expected to be handled by a try-catch in the call stack",
		);
	}
}

export type OpeningContext = {
	name: string;
	start: Position;
	indent: Number0;
	open: TokenType;
	close: TokenType;
};

export type ScopeType =
	| "FUNCTION_LOC"
	| "NON_ARROW_FUNCTION"
	| "FUNCTION"
	| "GENERATOR"
	| "ASYNC"
	| "PROPERTY_NAME"
	| "CLASS_PROPERTY"
	| "PARAMETERS"
	| "METHOD"
	| "CLASS"
	| "TYPE"
	| "MAX_NEW_DIAGNOSTICS"
	| "STRICT"
	| "FLOW_COMMENT";

const SCOPE_TYPES: Array<ScopeType> = [
	"FUNCTION_LOC",
	"NON_ARROW_FUNCTION",
	"FUNCTION",
	"GENERATOR",
	"ASYNC",
	"PROPERTY_NAME",
	"CLASS_PROPERTY",
	"PARAMETERS",
	"METHOD",
	"CLASS",
	"TYPE",
	"MAX_NEW_DIAGNOSTICS",
	"STRICT",
	"FLOW_COMMENT",
];

type JSParserMeta = {
	isLookahead: boolean;
	inModule: boolean;
	parenthesized: Set<string>;
	syntax: Set<ConstJSProgramSyntax>;
};

type JSParserTypes = {
	tokens: BaseTokens;
	state: State;
	options: JSParserOptions;
	meta: JSParserMeta;
};

export type JSParser = ParserCore<JSParserTypes>;

const EMPTY_POS: Position = {
	line: ob1Number1,
	column: ob1Number0,
};

export const createJSParser = createParser<JSParserTypes>({
	diagnosticCategory: "parse/js",
	getInitialState(): State {
		return {
			scopes: {},
			hasHoistedVars: false,
			tokens: [],
			potentialArrowAt: ob1Number0Neg1,
			commaAfterSpreadAt: ob1Number0Neg1,
			yieldPos: ob1Number0,
			awaitPos: ob1Number0,
			noArrowAt: [],
			noArrowParamsConversionAt: [],
			maybeInArrowParameters: false,
			isIterator: false,
			noAnonFunctionType: false,
			classLevel: ob1Number0,
			labels: [],
			yieldInPossibleArrowParameters: undefined,
			index: ob1Number0,
			lineStartIndex: ob1Number0,
			curLine: ob1Number1,
			tokenType: tt.eof,
			tokenValue: undefined,
			startPos: EMPTY_POS,
			endPos: EMPTY_POS,
			lastStartPos: EMPTY_POS,
			lastEndPos: EMPTY_POS,
			startIndex: ob1Number0,
			endIndex: ob1Number0,
			lastEndIndex: ob1Number0,
			lastStartIndex: ob1Number0,
			context: [ct.braceStatement],
			exprAllowed: true,
			containsOctal: false,
			escapePosition: undefined,
			octalPosition: undefined,
			invalidTemplateEscapePosition: undefined,
			exportedIdentifiers: new Map(),
			lineStart: true,
			indentLevel: ob1Number0,
		};
	},

	overrides: {
		getPosition(parser): Position {
			return parser.state.startPos;
		},

		getIndex(parser): Number0 {
			return parser.state.startIndex;
		},

		getLastEndPosition(parser): Position {
			return parser.state.lastEndPos;
		},
	},
});

export function resetTokenizerLine(parser: JSParser) {
	const {state} = parser;
	state.lineStartIndex = state.index;
	state.lineStart = true;
	state.indentLevel = ob1Number0;
}

export function getScope(parser: JSParser, type: ScopeType) {
	let scope = parser.state.scopes[type];
	if (scope === undefined) {
		scope = [];
		parser.state.scopes[type] = scope;
	}
	return scope;
}

export function getLastScope(parser: JSParser, type: ScopeType): unknown {
	const scope = getScope(parser, type);
	return scope[scope.length - 1];
}

export function pushScope(parser: JSParser, type: ScopeType, value?: unknown) {
	getScope(parser, type).push(value);
}

export function popScope(parser: JSParser, type: ScopeType) {
	getScope(parser, type).pop();
}

export function inScope(parser: JSParser, type: ScopeType): boolean {
	return hasScope(parser, type) && getLastScope(parser, type) !== false;
}

export function hasScope(parser: JSParser, type: ScopeType): boolean {
	const scope = parser.state.scopes[type];
	return scope !== undefined && scope.length > 0;
}

export function addParenthesized(parser: JSParser, node: AnyNode) {
	parser.meta.parenthesized.add(derivePositionKey(parser.getLoc(node).start));
}

export function isParenthesized(parser: JSParser, node: AnyNode): boolean {
	return parser.meta.parenthesized.has(
		derivePositionKey(parser.getLoc(node).start),
	);
}

export function setState(parser: JSParser, newState: ParserCoreState & State) {
	// Verify that this new state doesn't exceed any previous maxDiagnostic cap
	// maxDiagnostics will be at -1 when it's own limit has been exceeded, in
	// this case, we are likely replacing the State with another that's valid
	// and doesn't exceed
	const maxDiagnostics = getLastScope(parser, "MAX_NEW_DIAGNOSTICS");
	if (typeof maxDiagnostics === "number" && maxDiagnostics !== -1) {
		const diff = newState.diagnostics.length - parser.state.diagnostics.length;
		if (diff > maxDiagnostics) {
			throw new DiagnosticsFatalError();
		}
	}

	parser.state = newState;
}

export function atEOF(parser: JSParser): boolean {
	return match(parser, tt.eof);
}

export function createBranch<T>(parser: JSParser): ParserBranchFinder<T> {
	return new ParserBranchFinder(parser);
}

export function tryBranch<T>(
	parser: JSParser,
	fn: (parser: JSParser) => T,
): undefined | T {
	const branch = new ParserBranchFinder<T>(parser);
	branch.add(fn, {maxNewDiagnostics: 0});
	if (branch.hasBranch()) {
		return branch.pickOptional();
	} else {
		return undefined;
	}
}

export function createUnknownIdentifier(
	parser: JSParser,
	reason: string,
	start: Position = parser.getPosition(),
	end: Position = parser.getLastEndPosition(),
): JSIdentifier {
	parser.state.corrupt = true;
	return {
		type: "JSIdentifier",
		name: "INVALID_PLACEHOLDER",
		loc: parser.finishLocAt(start, end),
	};
}

export function assertNoSpace(
	parser: JSParser,
	_metadata: Omit<DiagnosticDescription, "category"> = descriptions.JS_PARSER.UNEXPECTED_SPACE,
): void {
	const {state} = parser;
	if (comparePositions(state.startPos, state.lastEndPos) === 1) {
		unexpectedDiagnostic(
			parser,
			{
				start: state.lastEndPos,
				end: state.lastEndPos,
				description: _metadata,
			},
		);
	}
}

export function shouldCreateToken(parser: JSParser): boolean {
	return parser.options.tokens && !parser.meta.isLookahead;
}

export function createToken(parser: JSParser, state: State): Token {
	const token: Token = {
		type: state.tokenType,
		loc: {
			filename: parser.filename,
			start: state.startPos,
			end: state.endPos,
		},
	};
	pushToken(parser, token);
	return token;
}

export function pushToken(parser: JSParser, token: Token) {
	const lastToken = parser.state.tokens[parser.state.tokens.length - 1];
	if (lastToken !== undefined) {
		if (comparePositions(token.loc.start, lastToken.loc.end) === -1) {
			throw new Error(
				"Trying to push a token that appears before the last pushed token",
			);
		}
	}

	parser.state.tokens.push(token);
}

export function unexpectedDiagnostic(
	parser: JSParser,
	opts: ParserUnexpectedOptions,
): void {
	if (parser.meta.isLookahead) {
		return;
	}

	let maxDiagnostics = getLastScope(parser, "MAX_NEW_DIAGNOSTICS");
	if (typeof maxDiagnostics === "number") {
		maxDiagnostics--;
		popScope(parser, "MAX_NEW_DIAGNOSTICS");
		pushScope(parser, "MAX_NEW_DIAGNOSTICS", maxDiagnostics);
		if (maxDiagnostics < 0) {
			throw new DiagnosticsFatalError();
		}
	}

	parser.unexpectedDiagnostic(opts);
}

export function shouldTokenizeJSX(parser: JSParser): boolean {
	return !isSyntaxEnabled(parser, "ts") || isSyntaxEnabled(parser, "jsx");
}

export function isSyntaxEnabled(
	parser: JSParser,
	syntax: ConstJSProgramSyntax,
): boolean {
	return parser.meta.syntax.has(syntax);
}

export function expectSyntaxEnabled(
	parser: JSParser,
	syntax: ConstJSProgramSyntax,
) {
	if (!isSyntaxEnabled(parser, syntax)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_ENABLE_SYNTAX(syntax),
			},
		);
	}
}

export function isRelational(parser: JSParser, op: "<" | ">"): boolean {
	return match(parser, tt.relational) && parser.state.tokenValue === op;
}

export function expectRelational(parser: JSParser, op: "<" | ">"): boolean {
	if (eatRelational(parser, op)) {
		return true;
	} else {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_RELATIONAL_OPERATOR,
			},
		);
		return false;
	}
}

export function banUnicodeEscape(
	parser: JSParser,
	index: undefined | Number0,
	name: string,
) {
	if (index !== undefined) {
		unexpectedDiagnostic(
			parser,
			{
				index,
				description: descriptions.JS_PARSER.ESCAPE_SEQUENCE_IN_WORD(name),
			},
		);
	}
}

// eat() for relational operators.
export function eatRelational(parser: JSParser, op: "<" | ">"): boolean {
	if (isRelational(parser, op)) {
		next(parser);
		return true;
	} else {
		return false;
	}
}

// Tests whether parsed token is a contextual keyword.
export function isContextual(parser: JSParser, name: string): boolean {
	return (
		match(parser, tt.name) &&
		parser.state.tokenValue === name &&
		parser.state.escapePosition === undefined
	);
}

export function isLookaheadContextual(parser: JSParser, name: string): boolean {
	const l = lookaheadState(parser);
	return (
		l.tokenType === tt.name &&
		l.tokenValue === name &&
		l.escapePosition === undefined
	);
}

// Consumes contextual keyword if possible.
export function eatContextual(parser: JSParser, name: string): boolean {
	if (isContextual(parser, name)) {
		next(parser);
		return true;
	} else {
		return false;
	}
}

// Asserts that following token is given contextual keyword.
export function expectContextual(
	parser: JSParser,
	name: string,
	_metadata: OptionalProps<DiagnosticDescription, "category"> = descriptions.JS_PARSER.EXPECTED_KEYWORD(
		name,
	),
): boolean {
	if (eatContextual(parser, name)) {
		return true;
	} else {
		unexpectedDiagnostic(
			parser,
			{
				description: _metadata,
			},
		);
		return false;
	}
}

// Test whether a semicolon can be inserted at the current position.
export function canInsertSemicolon(parser: JSParser): boolean {
	return (
		match(parser, tt.eof) ||
		match(parser, tt.braceR) ||
		hasPrecedingLineBreak(parser)
	);
}

export function hasPrecedingLineBreak(parser: JSParser): boolean {
	return lineBreak.test(
		parser.getRawInput(
			parser.getIndexFromPosition(parser.state.lastEndPos, parser.filename),
			parser.getIndexFromPosition(parser.state.startPos, parser.filename),
		),
	);
}

export function isLineTerminator(parser: JSParser): boolean {
	return eat(parser, tt.semi) || canInsertSemicolon(parser);
}

// Consume a semicolon, or, failing that, see if we are allowed to
// pretend that there is a semicolon at this position.
export function semicolon(parser: JSParser): void {
	if (!isLineTerminator(parser)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_SEMI_OR_LINE_TERMINATOR,
			},
		);
	}
}

// Expect a token of a given type. If found, consume it, otherwise,
// raise an unexpected token error at given pos.
export function expect(
	parser: JSParser,
	type: TokenType,
	pos?: Position,
): boolean {
	if (eat(parser, type)) {
		return true;
	} else {
		unexpectedToken(parser, pos, type);
		return false;
	}
}

export function expectOpening(
	parser: JSParser,
	open: TokenType,
	close: TokenType,
	name: string,
): OpeningContext {
	const pos = parser.getPosition();
	const indent = parser.state.indentLevel;
	expect(parser, open);
	return {
		indent,
		start: pos,
		name,
		open,
		close,
	};
}

export function expectClosing(parser: JSParser, context: OpeningContext) {
	if (match(parser, context.close)) {
		next(parser);
		return true;
	} else {
		const currPos = parser.getPosition();

		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_CLOSING(
					context.name,
					context.close.label,
					{
						filename: parser.filename,
						start: currPos,
						end: currPos,
					},
				),
				start: context.start,
				end: context.start,
			},
		);

		return false;
	}
}

// Raise an unexpected token error. Can take the expected token type
// instead of a message string.
export function unexpectedToken(
	parser: JSParser,
	pos?: Position,
	tokenType?: TokenType,
) {
	let expectedToken: undefined | string;
	let possibleShiftMistake: boolean = false;

	if (tokenType !== undefined) {
		expectedToken = tokenType.label;

		const possibleMistake = TOKEN_MISTAKES[tokenType.label];
		possibleShiftMistake =
			possibleMistake !== undefined &&
			possibleMistake === parser.state.tokenType.label;
	}

	unexpectedDiagnostic(
		parser,
		{
			description: descriptions.JS_PARSER.UNEXPECTED_TOKEN(
				expectedToken,
				possibleShiftMistake,
			),
			start: pos ?? parser.state.startPos,
			end: pos ?? parser.state.endPos,
		},
	);
}

export function cloneNode<T extends AnyNode>(parser: JSParser, node: T): T {
	if (
		node.leadingComments === undefined &&
		node.trailingComments === undefined &&
		node.innerComments === undefined
	) {
		return {
			// Do we really need to clone this?
			...node,
		};
	} else {
		return {
			...node,
			leadingComments: undefined,
			trailingComments: undefined,
			innerComments: undefined,
		};
	}
}

// Reset the start location of node to the start location of locationNode
export function resetStartLocationFromNode(
	parser: JSParser,
	node: AnyNode,
	locationNode: AnyNode,
): void {
	node.loc = {
		...parser.getLoc(node),
		start: parser.getLoc(locationNode).start,
	};
}

export function next(parser: JSParser): void {
	if (shouldCreateToken(parser)) {
		createToken(parser, parser.state);
	}

	parser.state.lastEndPos = parser.state.endPos;
	parser.state.lastStartPos = parser.state.startPos;
	parser.state.lastEndIndex = parser.state.endIndex;
	parser.state.lastStartIndex = parser.state.startIndex;
	nextToken(parser);
}

export function eat(parser: JSParser, type: TokenType): boolean {
	if (match(parser, type)) {
		next(parser);
		return true;
	}

	return false;
}

export function match(parser: JSParser, type: TokenType): boolean {
	return parser.state.tokenType === type;
}

export function lookaheadState(parser: JSParser): State {
	const old = parser.state;
	parser.state = cloneState(parser, true);

	parser.meta.isLookahead = true;
	next(parser);
	parser.meta.isLookahead = false;

	const curr = parser.state;
	parser.state = old;
	return curr;
}

export function cloneState(
	parser: JSParser,
	skipArrays: boolean = false,
): JSParser["state"] {
	const state: JSParser["state"] = {...parser.state};

	for (const key in state) {
		// @ts-ignore
		let val = state[key];

		const shouldSlice = !skipArrays || key === "context";
		if (shouldSlice && Array.isArray(val)) {
			// @ts-ignore
			state[key] = val.slice();
		}
	}

	const scopes = {...state.scopes};
	state.scopes = scopes;
	for (const type of SCOPE_TYPES) {
		const scope = scopes[type];
		if (scope !== undefined) {
			scopes[type] = scope.slice();
		}
	}

	return state;
}

// Private method to actually generate a Position
export function getPositionFromState(parser: JSParser): Position {
	const {state} = parser;
	const pos: Position = {
		line: state.curLine,
		column: ob1Sub(state.index, state.lineStartIndex),
	};
	parser.indexTracker.setPositionIndex(pos, state.index);
	return pos;
}

export function createMeta(opts: JSParserOptions): JSParserMeta {
	return {
		isLookahead: false,
		inModule: opts.sourceType === "template" || opts.sourceType === "module",
		parenthesized: new Set(),
		syntax: new Set(opts.syntax),
	};
}

export function parseRoot(parser: JSParser): JSRoot {
	if (parser.meta.inModule) {
		pushScope(parser, "ASYNC", true);
		pushScope(parser, "STRICT", true);
	}

	const program = parseTopLevel(parser);

	if (parser.meta.inModule) {
		popScope(parser, "ASYNC");
		popScope(parser, "STRICT");
	}

	// Smoke test for unpopped scopes
	for (const type of SCOPE_TYPES) {
		if (hasScope(parser, type)) {
			throw new Error(
				`Finished parsing but there was still a ${type} scope stack`,
			);
		}
	}

	// Smoke test for token exhaustion
	if (!match(parser, tt.eof)) {
		throw new Error("Finish parsing but we arent at the end of the file");
	}

	return program;
}
