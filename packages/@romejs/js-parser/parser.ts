/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ConstProgramSyntax,
	ConstSourceType,
	Identifier,
	Program,
	StringLiteral,
} from "@romejs/js-ast";
import {
	ParserOptionsWithRequiredPath,
	Position,
	SourceLocation,
	createParser,
} from "@romejs/parser-core";
import {JSParserOptions} from "./options";
import {
	DiagnosticDescription,
	DiagnosticFilter,
	DiagnosticLocation,
	Diagnostics,
	DiagnosticsProcessor,
	descriptions,
} from "@romejs/diagnostics";
import ParserBranchFinder from "./ParserBranchFinder";
import {Token, nextToken} from "./tokenizer/index";
import {TokenType, types as tt} from "./tokenizer/types";
import {lineBreak} from "@romejs/js-parser-utils";
import {parseTopLevel} from "./parser/index";
import {State, createInitialState} from "./tokenizer/state";
import {Number0, ob1Number0, ob1Sub} from "@romejs/ob1";
import {Dict, OptionalProps} from "@romejs/typescript-helpers";
import {attachComments} from "./parser/comments";
import CommentsConsumer from "./CommentsConsumer";

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

export const createJSParser = createParser((ParserCore, ParserWithRequiredPath) => {
	// rome-ignore lint/noExplicitAny
	class JSParser extends ParserWithRequiredPath<any, State> {
		constructor(options: JSParserOptions) {
			const state = createInitialState();

			const parserOpts: ParserOptionsWithRequiredPath = {
				path: options.path,
				mtime: options.mtime,
				input: options.input,
			};
			super(parserOpts, "parse/js", state);

			this.isTrackingTokens = options.tokens;

			this.isLookahead = false;

			this.sourceType = options.sourceType;
			this.options = options;
			this.inModule =
				this.options.sourceType === "template" ||
				this.options.sourceType === "module";
			this.parenthesized = new Set();
			this.comments = new CommentsConsumer();

			// Turn options.syntax into a Set, probably faster than doing `includes` on the array
			// We may also push stuff to it as we read comments such as `@\flow`
			this.syntax = new Set(options.syntax);
		}

		comments: CommentsConsumer;
		options: JSParserOptions;
		sourceType: ConstSourceType;
		syntax: Set<ConstProgramSyntax>;
		isTrackingTokens: boolean;
		inModule: boolean;
		isLookahead: boolean;
		parenthesized: Set<Number0>;

		resetTokenizerLine() {
			this.state.lineStartIndex = this.state.index;
			this.state.lineStart = true;
			this.state.indentLevel = ob1Number0;
		}

		getScope(type: ScopeType) {
			let scope = this.state.scopes[type];
			if (scope === undefined) {
				scope = [];
				this.state.scopes[type] = scope;
			}
			return scope;
		}

		getLastScope(type: ScopeType): unknown {
			const scope = this.getScope(type);
			return scope[scope.length - 1];
		}

		pushScope(type: ScopeType, value?: unknown) {
			//console.log('+' + type);
			//console.group();
			this.getScope(type).push(value);
		}

		popScope(type: ScopeType) {
			//console.groupEnd();
			//console.log('-' + type);
			this.getScope(type).pop();
		}

		inScope(type: ScopeType): boolean {
			return this.hasScope(type) && this.getLastScope(type) !== false;
		}

		hasScope(type: ScopeType): boolean {
			const scope = this.state.scopes[type];
			return scope !== undefined && scope.length > 0;
		}

		addParenthesized(node: AnyNode) {
			this.parenthesized.add(this.getLoc(node).start.index);
		}

		isParenthesized(node: AnyNode): boolean {
			return this.parenthesized.has(this.getLoc(node).start.index);
		}

		setState(newState: State) {
			// Verify that this new state doesn't exceed any previous maxDiagnostic cap
			// maxDiagnostics will be at -1 when it's own limit has been exceeded, in
			// this case, we are likely replacing the State with another that's valid
			// and doesn't exceed
			const maxDiagnostics = this.getLastScope("MAX_NEW_DIAGNOSTICS");
			if (typeof maxDiagnostics === "number" && maxDiagnostics !== -1) {
				const diff = newState.diagnostics.length - this.state.diagnostics.length;
				if (diff > maxDiagnostics) {
					throw new DiagnosticsFatalError();
				}
			}

			this.state = newState;
		}

		atEOF(): boolean {
			return this.match(tt.eof);
		}

		createBranch<T>(): ParserBranchFinder<T> {
			return new ParserBranchFinder(this);
		}

		tryBranch<T>(fn: (parser: JSParser) => T): undefined | T {
			const branch = new ParserBranchFinder<T>(this);
			branch.add(fn, {maxNewDiagnostics: 0});
			if (branch.hasBranch()) {
				return branch.pickOptional();
			} else {
				return undefined;
			}
		}

		finalizeNode<T extends AnyNode>(node: T): T {
			attachComments(this, node);
			return node;
		}

		// Sometimes we want to pretend we're in different locations to consume the comments of other nodes
		finishNodeWithCommentStarts<T extends AnyNode>(
			starts: Array<Position>,
			node: T,
		): T {
			for (const start of starts) {
				node = this.finishNode(start, node);
			}
			return node;
		}

		finishNode<T extends AnyNode>(start: Position, node: T): T {
			return this.finishNodeAt(start, this.getLastEndPosition(), node);
		}

		finishNodeAt<T extends AnyNode>(start: Position, end: Position, node: T): T {
			// Maybe mutating `node` is better...?
			const newNode: T = {
				...node,
				loc: this.finishLocAt(start, end),
			};
			return this.finalizeNode(newNode);
		}

		createUnknownIdentifier(
			reason: string,
			start: Position = this.getPosition(),
			end: Position = this.getLastEndPosition(),
		): Identifier {
			this.state.corrupt = true;
			return {
				type: "Identifier",
				name: "INVALID_PLACEHOLDER",
				loc: this.finishLocAt(start, end),
			};
		}

		createUnknownStringLiteral(
			reason: string,
			start: Position = this.getPosition(),
			end: Position = this.getLastEndPosition(),
		): StringLiteral {
			this.state.corrupt = true;
			return {
				type: "StringLiteral",
				value: "INVALID_PLACEHOLDER",
				loc: this.finishLocAt(start, end),
			};
		}

		assertNoSpace(
			_metadata: Omit<DiagnosticDescription, "category"> = descriptions.JS_PARSER.UNEXPECTED_SPACE,
		): void {
			const {state} = this;

			if (state.startPos.index > state.lastEndPos.index) {
				this.addDiagnostic({
					start: state.lastEndPos,
					end: state.lastEndPos,
					description: _metadata,
				});
			}
		}

		getDiagnostics(): Diagnostics {
			const collector = new DiagnosticsProcessor({
				origins: [
					{
						category: "js-parser",
					},
				],
				//unique: ['start.line'],
			});

			for (const filter of this.state.diagnosticFilters) {
				collector.addFilter(filter);
			}

			// TODO remove any trailing "eof" diagnostic
			return collector.addDiagnostics(this.state.diagnostics).slice(0, 1);
		}

		addDiagnosticFilter(diag: DiagnosticFilter) {
			this.state.diagnosticFilters.push(diag);
		}

		addCompleteDiagnostic(diags: Diagnostics) {
			this.state.diagnostics = [...this.state.diagnostics, ...diags];
		}

		shouldCreateToken() {
			return this.isTrackingTokens && this.isLookahead === false;
		}

		createToken(state: State): Token {
			const token: Token = {
				type: state.tokenType,
				start: state.startPos.index,
				end: state.endPos.index,
				loc: {
					filename: this.filename,
					start: state.startPos,
					end: state.endPos,
				},
			};
			this.pushToken(token);
			return token;
		}

		pushToken(token: Token) {
			const lastToken = this.state.tokens[this.state.tokens.length - 1];
			if (lastToken !== undefined) {
				if (token.loc.start.index < lastToken.loc.end.index) {
					throw new Error(
						"Trying to push a token that appears before the last pushed token",
					);
				}
			}

			this.state.tokens.push(token);
		}

		addDiagnostic(
			opts: {
				description: Omit<DiagnosticDescription, "category">;
				start?: Position;
				end?: Position;
				loc?: SourceLocation;
				index?: Number0;
				location?: DiagnosticLocation;
			},
		): void {
			if (this.isLookahead) {
				return;
			}

			let maxDiagnostics = this.getLastScope("MAX_NEW_DIAGNOSTICS");
			if (typeof maxDiagnostics === "number") {
				maxDiagnostics--;
				this.popScope("MAX_NEW_DIAGNOSTICS");
				this.pushScope("MAX_NEW_DIAGNOSTICS", maxDiagnostics);
				if (maxDiagnostics < 0) {
					throw new DiagnosticsFatalError();
				}
			}

			if (this.state.diagnostics.length > 0) {
				//return;
			}

			let {start, end} = opts;

			if (opts.index !== undefined) {
				start = this.getPositionFromIndex(opts.index);
				end = start;
			}

			if (opts.location !== undefined) {
				start = opts.location.start;
				end = opts.location.end;
			}

			if (start === undefined && end === undefined && opts.loc !== undefined) {
				start = opts.loc.start;
				end = opts.loc.end;
			}

			// If we weren't given a start then default to the provided end, or the current token start
			if (start === undefined && end === undefined) {
				start = this.getPosition();
				end = this.getLastEndPosition();
			}

			if (start === undefined && end !== undefined) {
				start = end;
			}

			if (start !== undefined && end === undefined) {
				end = start;
			}

			this.state.diagnostics.push({
				description: {
					category: "parse/js",
					...opts.description,
				},
				location: {
					filename: this.filename,
					sourceType: this.sourceType,
					mtime: this.mtime,
					start,
					end,
				},
			});
		}

		shouldTokenizeJSX(): boolean {
			return !this.isSyntaxEnabled("ts") || this.isSyntaxEnabled("jsx");
		}

		isSyntaxEnabled(syntax: ConstProgramSyntax): boolean {
			return this.syntax.has(syntax);
		}

		expectSyntaxEnabled(syntax: ConstProgramSyntax) {
			if (!this.isSyntaxEnabled(syntax)) {
				this.addDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_ENABLE_SYNTAX(syntax),
				});
			}
		}

		isRelational(op: "<" | ">"): boolean {
			return this.match(tt.relational) && this.state.tokenValue === op;
		}

		expectRelational(op: "<" | ">"): boolean {
			if (this.eatRelational(op)) {
				return true;
			} else {
				this.addDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_RELATIONAL_OPERATOR,
				});
				return false;
			}
		}

		isLookaheadRelational(op: "<" | ">"): boolean {
			const l = this.lookaheadState();
			return l.tokenType === tt.relational && l.tokenValue === op;
		}

		banUnicodeEscape(index: undefined | Number0, name: string) {
			if (index !== undefined) {
				this.addDiagnostic({
					index,
					description: descriptions.JS_PARSER.ESCAPE_SEQUENCE_IN_WORD(name),
				});
			}
		}

		// eat() for relational operators.
		eatRelational(op: "<" | ">"): boolean {
			if (this.isRelational(op)) {
				this.next();
				return true;
			} else {
				return false;
			}
		}

		// Tests whether parsed token is a contextual keyword.
		isContextual(name: string): boolean {
			return (
				this.match(tt.name) &&
				this.state.tokenValue === name &&
				this.state.escapePosition === undefined
			);
		}

		isLookaheadContextual(name: string): boolean {
			const l = this.lookaheadState();
			return (
				l.tokenType === tt.name &&
				l.tokenValue === name &&
				l.escapePosition === undefined
			);
		}

		// Consumes contextual keyword if possible.
		eatContextual(name: string): boolean {
			if (this.isContextual(name)) {
				this.next();
				return true;
			} else {
				return false;
			}
		}

		// Asserts that following token is given contextual keyword.
		expectContextual(
			name: string,
			_metadata: OptionalProps<DiagnosticDescription, "category"> = descriptions.JS_PARSER.EXPECTED_KEYWORD(
				name,
			),
		): boolean {
			if (this.eatContextual(name)) {
				return true;
			} else {
				this.addDiagnostic({
					description: _metadata,
				});
				return false;
			}
		}

		// Test whether a semicolon can be inserted at the current position.
		canInsertSemicolon(): boolean {
			return (
				this.match(tt.eof) || this.match(tt.braceR) || this.hasPrecedingLineBreak()
			);
		}

		hasPrecedingLineBreak(): boolean {
			return lineBreak.test(
				this.getRawInput(this.state.lastEndPos.index, this.state.startPos.index),
			);
		}

		isLineTerminator(): boolean {
			return this.eat(tt.semi) || this.canInsertSemicolon();
		}

		// Consume a semicolon, or, failing that, see if we are allowed to

		// pretend that there is a semicolon at this position.
		semicolon(): void {
			if (!this.isLineTerminator()) {
				this.addDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_SEMI_OR_LINE_TERMINATOR,
				});
			}
		}

		// Expect a token of a given type. If found, consume it, otherwise,

		// raise an unexpected token error at given pos.
		expect(type: TokenType, pos?: Position): boolean {
			if (this.eat(type)) {
				return true;
			} else {
				this.unexpectedToken(pos, type);
				return false;
			}
		}

		expectOpening(open: TokenType, close: TokenType, name: string): OpeningContext {
			const pos = this.getPosition();
			const indent = this.state.indentLevel;
			this.expect(open);
			return {
				indent,
				start: pos,
				name,
				open,
				close,
			};
		}

		expectClosing(context: OpeningContext) {
			if (this.match(context.close)) {
				this.next();
				return true;
			} else {
				const currPos = this.getPosition();

				this.addDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_CLOSING(
						context.name,
						context.close.label,
						{
							filename: this.filename,
							start: currPos,
							end: currPos,
						},
					),
					start: context.start,
					end: context.start,
				});

				return false;
			}
		}

		// Raise an unexpected token error. Can take the expected token type

		// instead of a message string.
		unexpectedToken(pos?: Position, tokenType?: TokenType) {
			let expectedToken: undefined | string;
			let possibleShiftMistake: boolean = false;

			if (tokenType !== undefined) {
				expectedToken = tokenType.label;

				const possibleMistake = TOKEN_MISTAKES[tokenType.label];
				possibleShiftMistake =
					possibleMistake !== undefined &&
					possibleMistake === this.state.tokenType.label;
			}

			this.addDiagnostic({
				description: descriptions.JS_PARSER.UNEXPECTED_TOKEN(
					expectedToken,
					possibleShiftMistake,
				),
				start: pos === undefined ? this.state.startPos : pos,
				end: pos === undefined ? this.state.endPos : pos,
			});
		}

		unexpected(): never {
			throw new Error(
				"js-parser should never throw an exception, use addDiagnostic or unexpectedToken instead",
			);
		}

		tokenize(): never {
			throw new Error("js-parser does not use the parser-core tokenizer");
		}

		cloneNode<T extends AnyNode>(node: T): T {
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
		resetStartLocationFromNode(node: AnyNode, locationNode: AnyNode): void {
			node.loc = {
				...this.getLoc(node),
				start: this.getLoc(locationNode).start,
			};
		}

		next(): void {
			if (this.shouldCreateToken()) {
				this.createToken(this.state);
			}

			this.state.lastEndPos = this.state.endPos;
			this.state.lastStartPos = this.state.startPos;
			nextToken(this);
		}

		eat(type: TokenType): boolean {
			if (this.match(type)) {
				this.next();
				return true;
			}

			return false;
		}

		match(type: TokenType): boolean {
			return this.state.tokenType === type;
		}

		lookaheadState(): State {
			const old = this.state;
			this.state = this.cloneState(true);

			this.isLookahead = true;
			this.next();
			this.isLookahead = false;

			const curr = this.state;
			this.state = old;
			return curr;
		}

		cloneState(skipArrays: boolean = false): State {
			const state: State = {...this.state};

			for (const key in state) {
				// @ts-ignore
				let val = state[key];

				const shouldSlice = skipArrays === false || key === "context";
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

		// Overrides ParserCore#getPosition
		getPosition(): Position {
			return this.state.startPos;
		}

		// Overrides ParserCore#getLastEndPosition
		getLastEndPosition(): Position {
			return this.state.lastEndPos;
		}

		// Private method to actually generate a Position
		getPositionFromState(): Position {
			const {state} = this;
			return {
				index: state.index,
				line: state.curLine,
				column: ob1Sub(state.index, state.lineStartIndex),
			};
		}

		parse(): Program {
			if (this.inModule) {
				this.pushScope("ASYNC", true);
				this.pushScope("STRICT", true);
			}

			const program = parseTopLevel(this);

			if (this.inModule) {
				this.popScope("ASYNC");
				this.popScope("STRICT");
			}

			// Smoke test for unpopped scopes
			for (const type of SCOPE_TYPES) {
				if (this.hasScope(type)) {
					throw new Error(
						`Finished parsing but there was still a ${type} scope stack`,
					);
				}
			}

			// Smoke test for token exhaustion
			if (!this.match(tt.eof)) {
				throw new Error("Finish parsing but we arent at the end of the file");
			}

			return program;
		}
	}

	return JSParser;
});

export type JSParser = ReturnType<typeof createJSParser>;
