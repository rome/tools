/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ConstJSProgramSyntax,
	ConstJSSourceType,
	JSIdentifier,
	JSRoot,
} from "@internal/ast";
import {
	ParserOptionsWithRequiredPath,
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
import {State, createInitialState} from "./tokenizer/state";
import {Number0, ob1Number0, ob1Sub} from "@internal/ob1";
import {Dict, OptionalProps} from "@internal/typescript-helpers";

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
	// rome-ignore lint/ts/noExplicitAny
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

			// Turn options.syntax into a Set, probably faster than doing `includes` on the array
			// We may also push stuff to it as we read comments such as `@\flow`
			this.syntax = new Set(options.syntax);
		}

		public options: JSParserOptions;
		public sourceType: ConstJSSourceType;
		public syntax: Set<ConstJSProgramSyntax>;
		private isTrackingTokens: boolean;
		public inModule: boolean;
		public isLookahead: boolean;
		private parenthesized: Set<string>;

		public resetTokenizerLine() {
			this.state.lineStartIndex = this.state.index;
			this.state.lineStart = true;
			this.state.indentLevel = ob1Number0;
		}

		private getScope(type: ScopeType) {
			let scope = this.state.scopes[type];
			if (scope === undefined) {
				scope = [];
				this.state.scopes[type] = scope;
			}
			return scope;
		}

		public getLastScope(type: ScopeType): unknown {
			const scope = this.getScope(type);
			return scope[scope.length - 1];
		}

		public pushScope(type: ScopeType, value?: unknown) {
			this.getScope(type).push(value);
		}

		public popScope(type: ScopeType) {
			this.getScope(type).pop();
		}

		public inScope(type: ScopeType): boolean {
			return this.hasScope(type) && this.getLastScope(type) !== false;
		}

		public hasScope(type: ScopeType): boolean {
			const scope = this.state.scopes[type];
			return scope !== undefined && scope.length > 0;
		}

		public addParenthesized(node: AnyNode) {
			this.parenthesized.add(derivePositionKey(this.getLoc(node).start));
		}

		public isParenthesized(node: AnyNode): boolean {
			return this.parenthesized.has(derivePositionKey(this.getLoc(node).start));
		}

		public setState(newState: State) {
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

		public atEOF(): boolean {
			return this.match(tt.eof);
		}

		public createBranch<T>(): ParserBranchFinder<T> {
			return new ParserBranchFinder(this);
		}

		public tryBranch<T>(fn: (parser: JSParser) => T): undefined | T {
			const branch = new ParserBranchFinder<T>(this);
			branch.add(fn, {maxNewDiagnostics: 0});
			if (branch.hasBranch()) {
				return branch.pickOptional();
			} else {
				return undefined;
			}
		}

		public createUnknownIdentifier(
			reason: string,
			start: Position = this.getPosition(),
			end: Position = this.getLastEndPosition(),
		): JSIdentifier {
			this.state.corrupt = true;
			return {
				type: "JSIdentifier",
				name: "INVALID_PLACEHOLDER",
				loc: this.finishLocAt(start, end),
			};
		}

		public assertNoSpace(
			_metadata: Omit<DiagnosticDescription, "category"> = descriptions.JS_PARSER.UNEXPECTED_SPACE,
		): void {
			const {state} = this;

			if (comparePositions(state.startPos, state.lastEndPos) === 1) {
				this.unexpectedDiagnostic({
					start: state.lastEndPos,
					end: state.lastEndPos,
					description: _metadata,
				});
			}
		}

		public shouldCreateToken() {
			return this.isTrackingTokens && !this.isLookahead;
		}

		private createToken(state: State): Token {
			const token: Token = {
				type: state.tokenType,
				loc: {
					filename: this.filename,
					start: state.startPos,
					end: state.endPos,
				},
			};
			this.pushToken(token);
			return token;
		}

		public pushToken(token: Token) {
			const lastToken = this.state.tokens[this.state.tokens.length - 1];
			if (lastToken !== undefined) {
				if (comparePositions(token.loc.start, lastToken.loc.end) === -1) {
					throw new Error(
						"Trying to push a token that appears before the last pushed token",
					);
				}
			}

			this.state.tokens.push(token);
		}

		public unexpectedDiagnostic(opts: ParserUnexpectedOptions): void {
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

			super.unexpectedDiagnostic(opts);
		}

		public shouldTokenizeJSX(): boolean {
			return !this.isSyntaxEnabled("ts") || this.isSyntaxEnabled("jsx");
		}

		public isSyntaxEnabled(syntax: ConstJSProgramSyntax): boolean {
			return this.syntax.has(syntax);
		}

		public expectSyntaxEnabled(syntax: ConstJSProgramSyntax) {
			if (!this.isSyntaxEnabled(syntax)) {
				this.unexpectedDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_ENABLE_SYNTAX(syntax),
				});
			}
		}

		public isRelational(op: "<" | ">"): boolean {
			return this.match(tt.relational) && this.state.tokenValue === op;
		}

		public expectRelational(op: "<" | ">"): boolean {
			if (this.eatRelational(op)) {
				return true;
			} else {
				this.unexpectedDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_RELATIONAL_OPERATOR,
				});
				return false;
			}
		}

		public banUnicodeEscape(index: undefined | Number0, name: string) {
			if (index !== undefined) {
				this.unexpectedDiagnostic({
					index,
					description: descriptions.JS_PARSER.ESCAPE_SEQUENCE_IN_WORD(name),
				});
			}
		}

		// eat() for relational operators.
		private eatRelational(op: "<" | ">"): boolean {
			if (this.isRelational(op)) {
				this.next();
				return true;
			} else {
				return false;
			}
		}

		// Tests whether parsed token is a contextual keyword.
		public isContextual(name: string): boolean {
			return (
				this.match(tt.name) &&
				this.state.tokenValue === name &&
				this.state.escapePosition === undefined
			);
		}

		public isLookaheadContextual(name: string): boolean {
			const l = this.lookaheadState();
			return (
				l.tokenType === tt.name &&
				l.tokenValue === name &&
				l.escapePosition === undefined
			);
		}

		// Consumes contextual keyword if possible.
		public eatContextual(name: string): boolean {
			if (this.isContextual(name)) {
				this.next();
				return true;
			} else {
				return false;
			}
		}

		// Asserts that following token is given contextual keyword.
		public expectContextual(
			name: string,
			_metadata: OptionalProps<DiagnosticDescription, "category"> = descriptions.JS_PARSER.EXPECTED_KEYWORD(
				name,
			),
		): boolean {
			if (this.eatContextual(name)) {
				return true;
			} else {
				this.unexpectedDiagnostic({
					description: _metadata,
				});
				return false;
			}
		}

		// Test whether a semicolon can be inserted at the current position.
		public canInsertSemicolon(): boolean {
			return (
				this.match(tt.eof) ||
				this.match(tt.braceR) ||
				this.hasPrecedingLineBreak()
			);
		}

		public hasPrecedingLineBreak(): boolean {
			return lineBreak.test(
				this.getRawInput(
					this.getIndexFromPosition(this.state.lastEndPos, this.filename),
					this.getIndexFromPosition(this.state.startPos, this.filename),
				),
			);
		}

		public isLineTerminator(): boolean {
			return this.eat(tt.semi) || this.canInsertSemicolon();
		}

		// Consume a semicolon, or, failing that, see if we are allowed to
		// pretend that there is a semicolon at this position.
		public semicolon(): void {
			if (!this.isLineTerminator()) {
				this.unexpectedDiagnostic({
					description: descriptions.JS_PARSER.EXPECTED_SEMI_OR_LINE_TERMINATOR,
				});
			}
		}

		// Expect a token of a given type. If found, consume it, otherwise,
		// raise an unexpected token error at given pos.
		public expect(type: TokenType, pos?: Position): boolean {
			if (this.eat(type)) {
				return true;
			} else {
				this.unexpectedToken(pos, type);
				return false;
			}
		}

		public expectOpening(
			open: TokenType,
			close: TokenType,
			name: string,
		): OpeningContext {
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

		public expectClosing(context: OpeningContext) {
			if (this.match(context.close)) {
				this.next();
				return true;
			} else {
				const currPos = this.getPosition();

				this.unexpectedDiagnostic({
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
		public unexpectedToken(pos?: Position, tokenType?: TokenType) {
			let expectedToken: undefined | string;
			let possibleShiftMistake: boolean = false;

			if (tokenType !== undefined) {
				expectedToken = tokenType.label;

				const possibleMistake = TOKEN_MISTAKES[tokenType.label];
				possibleShiftMistake =
					possibleMistake !== undefined &&
					possibleMistake === this.state.tokenType.label;
			}

			this.unexpectedDiagnostic({
				description: descriptions.JS_PARSER.UNEXPECTED_TOKEN(
					expectedToken,
					possibleShiftMistake,
				),
				start: pos ?? this.state.startPos,
				end: pos ?? this.state.endPos,
			});
		}

		public unexpected(): never {
			throw new Error(
				"js-parser should never throw an exception, use addDiagnostic or unexpectedToken instead",
			);
		}

		protected tokenize(): never {
			throw new Error("js-parser does not use the parser-core tokenizer");
		}

		public cloneNode<T extends AnyNode>(node: T): T {
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
		public resetStartLocationFromNode(
			node: AnyNode,
			locationNode: AnyNode,
		): void {
			node.loc = {
				...this.getLoc(node),
				start: this.getLoc(locationNode).start,
			};
		}

		public next(): void {
			if (this.shouldCreateToken()) {
				this.createToken(this.state);
			}

			this.state.lastEndPos = this.state.endPos;
			this.state.lastStartPos = this.state.startPos;
			this.state.lastEndIndex = this.state.endIndex;
			this.state.lastStartIndex = this.state.startIndex;
			nextToken(this);
		}

		public eat(type: TokenType): boolean {
			if (this.match(type)) {
				this.next();
				return true;
			}

			return false;
		}

		public match(type: TokenType): boolean {
			return this.state.tokenType === type;
		}

		public lookaheadState(): State {
			const old = this.state;
			this.state = this.cloneState(true);

			this.isLookahead = true;
			this.next();
			this.isLookahead = false;

			const curr = this.state;
			this.state = old;
			return curr;
		}

		public cloneState(skipArrays: boolean = false): State {
			const state: State = {...this.state};

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

		// Overrides ParserCore#getPosition
		public getPosition(): Position {
			return this.state.startPos;
		}

		// Overrides ParserCore#getIndex
		public getIndex(): Number0 {
			return this.state.startIndex;
		}

		// Overrides ParserCore#getLastEndPosition
		public getLastEndPosition(): Position {
			return this.state.lastEndPos;
		}

		// Private method to actually generate a Position
		public getPositionFromState(): Position {
			const {state} = this;
			const pos: Position = {
				line: state.curLine,
				column: ob1Sub(state.index, state.lineStartIndex),
			};
			this.indexTracker.setPositionIndex(pos, state.index);
			return pos;
		}

		public parse(): JSRoot {
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
