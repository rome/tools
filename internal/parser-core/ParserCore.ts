import {
	ComplexToken,
	EOFToken,
	NodeBase,
	ParserOptions,
	ParserOptionsWithRequiredPath,
	ParserUnexpectedOptions,
	Position,
	SOFToken,
	SimpleToken,
	SourceLocation,
	TokenBase,
	TokenValues,
	TokensShape,
	ValueToken,
} from "./types";
import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticFilter,
	DiagnosticLocation,
	Diagnostics,
	DiagnosticsError,
	DiagnosticsProcessor,
	catchDiagnosticsSync,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {AnyComment, AnyNode, RootBase} from "@internal/ast";
import {UnknownFilePath, createUnknownFilePath} from "@internal/path";
import {
	Number0,
	Number1,
	ob1Add,
	ob1Coerce0,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number1,
} from "@internal/ob1";

import {CommentsConsumer} from "@internal/js-parser";
import PositionTracker from "./PositionTracker";
import {RequiredProps} from "@internal/typescript-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {attachComments} from "./comments";

export type ParserCoreState = {
	comments: Array<AnyComment>;
	trailingComments: Array<AnyComment>;
	leadingComments: Array<AnyComment>;
	commentStack: Array<AnyNode>;
	commentPreviousNode: undefined | AnyNode;
	diagnostics: Diagnostics;
	diagnosticFilters: Array<DiagnosticFilter>;
	corrupt: boolean;
};

export default class ParserCore<
	Tokens extends TokensShape,
	State extends ParserCoreState = ParserCoreState
> {
	constructor(
		opts: ParserOptions,
		diagnosticCategory: DiagnosticCategory,
		initialState: Omit<State, keyof ParserCoreState>,
	) {
		const {
			path,
			mtime,
			offsetPosition,
			sourceText,
		} = opts;

		// Input information
		this.path = path === undefined ? undefined : createUnknownFilePath(path);
		this.filename = this.path === undefined ? undefined : this.path.join();
		this.mtime = mtime;
		this.input = normalizeInput(opts);
		this.sourceText = sourceText ?? this.input;
		this.length = ob1Coerce0(this.input.length);

		this.eofToken = {
			type: "EOF",
			start: ob1Coerce0(this.input.length),
			end: ob1Coerce0(this.input.length),
		};

		// Parser/tokenizer state
		this.diagnosticCategory = diagnosticCategory;
		this.tokenizing = false;
		this.currLine =
			offsetPosition === undefined ? ob1Number1 : offsetPosition.line;
		this.currColumn =
			offsetPosition === undefined ? ob1Number0 : offsetPosition.column;
		this.nextTokenIndex = ob1Number0;
		this.currentToken = SOF_TOKEN;
		this.prevToken = SOF_TOKEN;
		this.ignoreWhitespaceTokens = false;
		this.comments = new CommentsConsumer();

		this.indexTracker = new PositionTracker({
			filename: this.filename,
			input: this.input,
			offsetPosition,
			getPosition: this.getPosition.bind(this),
		});

		// @ts-ignore
		this.state = {
			...initialState,
			...ParserCore.createInitialState(),
		};
	}

	comments: CommentsConsumer;
	indexTracker: PositionTracker;
	diagnosticCategory: DiagnosticCategory;
	tokenizing: boolean;
	nextTokenIndex: Number0;
	state: State;
	prevToken: TokenValues<Tokens>;
	currentToken: TokenValues<Tokens>;
	eofToken: EOFToken;
	ignoreWhitespaceTokens: boolean;

	path: undefined | UnknownFilePath;
	filename: undefined | string;
	mtime: undefined | number;
	input: string;
	sourceText: string;
	length: Number0;
	currLine: Number1;
	currColumn: Number0;

	static createInitialState(): ParserCoreState {
		return {
			corrupt: false,
			trailingComments: [],
			leadingComments: [],
			commentStack: [],
			comments: [],
			commentPreviousNode: undefined,
			diagnostics: [],
			diagnosticFilters: [],
		};
	}

	getPathAssert(): UnknownFilePath {
		const {path} = this;
		if (path === undefined) {
			throw new Error("Path expected but none was passed to this Parser");
		} else {
			return path;
		}
	}

	getFilenameAssert(): string {
		const {filename} = this;
		if (filename === undefined) {
			throw new Error("Filename expected but none was passed to this Parser");
		} else {
			return filename;
		}
	}

	// Run the tokenizer over all tokens
	tokenizeAll(): Array<TokenValues<Tokens>> {
		const tokens: Array<TokenValues<Tokens>> = [];

		const {diagnostics} = catchDiagnosticsSync(() => {
			while (true) {
				tokens.push(this.getToken());
				if (this.matchToken("EOF")) {
					break;
				}

				this.nextToken();
			}
		});

		if (diagnostics !== undefined) {
			tokens.push({
				type: "Invalid",
				start: this.nextTokenIndex,
				end: this.length,
			});
		}

		return tokens;
	}

	// Tokenize method that must be implemented by subclasses
	tokenize(index: Number0): undefined | TokenValues<Tokens> {
		throw new Error("Unimplemented");
	}

	// Alternate tokenize method to allow that allows the use of state
	tokenizeWithState(
		index: Number0,
		state: State,
	):
		| undefined
		| {
				token: TokenValues<Tokens>;
				state: State;
			} {
		const token = this.tokenize(index);
		if (token !== undefined) {
			return {token, state};
		} else {
			return undefined;
		}
	}

	_tokenizeWithState(
		index: Number0,
		state: State,
	):
		| undefined
		| {
				token: TokenValues<Tokens>;
				state: State;
			} {
		if (this.ignoreWhitespaceTokens) {
			switch (this.getInputCharOnly(index)) {
				case " ":
				case "\t":
				case "\r":
				case "\n":
					return this.lookahead(ob1Inc(index));
			}
		}

		return this.tokenizeWithState(index, state);
	}

	// Get the current token
	getToken(): TokenValues<Tokens> {
		const {currentToken} = this;
		if (currentToken === SOF_TOKEN) {
			return this.nextToken();
		} else {
			return currentToken;
		}
	}

	getPrevToken(): TokenValues<Tokens> {
		return this.prevToken;
	}

	save(): ParserSnapshot<Tokens, State> {
		return {
			nextTokenIndex: this.nextTokenIndex,
			currentToken: this.currentToken,
			prevToken: this.prevToken,
			state: this.state,
		};
	}

	restore(snapshot: ParserSnapshot<Tokens, State>) {
		this.nextTokenIndex = snapshot.nextTokenIndex;
		this.currentToken = snapshot.currentToken;
		this.prevToken = snapshot.prevToken;
		this.state = snapshot.state;
	}

	// Advance to the next token, returning the new one
	nextToken(): TokenValues<Tokens> {
		if (this.isEOF(this.nextTokenIndex)) {
			this.currentToken = this.eofToken;
			return this.eofToken;
		}

		if (this.tokenizing) {
			throw new Error("Can't call nextToken while tokenizing");
		}

		const prevToken = this.currentToken;
		const {token: nextToken, state} = this.lookahead();

		if (nextToken.end === prevToken.end) {
			throw new Error(
				`tokenize() returned a token with the same position as the last - Previous token: ${JSON.stringify(
					prevToken,
				)}; Next token: ${JSON.stringify(nextToken)}; Input: ${this.input.slice(
					0,
					100,
				)}`,
			);
		}

		const {line, column} = this.getPositionFromIndex(nextToken.start);
		this.currLine = line;
		this.currColumn = column;

		this.nextTokenIndex = nextToken.end;
		this.prevToken = prevToken;
		this.currentToken = nextToken;
		this.state = state;
		return nextToken;
	}

	// Get the start index of the current token
	getIndex(): Number0 {
		return this.currentToken.start;
	}

	// Get the position of the current token
	getPosition(): Position {
		const index = this.getIndex();
		const cached = this.indexTracker.cachedPositions.get(index);
		if (cached !== undefined) {
			return cached;
		}

		const pos: Position = {
			line: this.currLine,
			column: this.currColumn,
		};
		this.indexTracker.setPositionIndex(pos, index);
		return pos;
	}

	// Get the end position of the previous token
	getLastEndPosition(): Position {
		return this.getPositionFromIndex(this.prevToken.end);
	}

	// Return the token that's after this current token without advancing to it
	lookaheadToken(index?: Number0): TokenValues<Tokens> {
		return this.lookahead(index).token;
	}

	// Return the token and state that's after the current token without advancing to it
	lookahead(
		index: Number0 = this.nextTokenIndex,
	): {
		token: TokenValues<Tokens>;
		state: State;
	} {
		if (this.isEOF(index)) {
			return {token: this.eofToken, state: this.state};
		}

		// Set the next token index, in the case of a lookahead we'll set it back later
		const prevNextTokenIndex = this.nextTokenIndex;
		this.nextTokenIndex = index;

		// Indicate that we're currently tokenizing to catch some weird recursive tokenizing errors
		const wasTokenizing = this.tokenizing;
		this.tokenizing = true;

		// Tokenize and do some validation
		const nextToken = this._tokenizeWithState(index, this.state);
		if (nextToken === undefined) {
			throw this.unexpected({
				start: this.getPositionFromIndex(index),
			});
		}

		// Reset to old values
		this.tokenizing = wasTokenizing;
		this.nextTokenIndex = prevNextTokenIndex;

		return nextToken;
	}

	getPositionFromIndex(index: Number0): Position {
		return this.indexTracker.getPositionFromIndex(index);
	}

	getIndexFromPosition(pos: Position, filename: undefined | string): Number0 {
		return this.indexTracker.getIndexFromPosition(pos, filename);
	}

	createDiagnostic(opts: ParserUnexpectedOptions = {}): Diagnostic {
		const {currentToken} = this;
		let {description} = opts;
		const location = this.getDiagnosticLocation(opts);
		const start = this.getIndexFromPosition(location.start, location.filename);

		// Normalize message, we need to be defensive here because it could have been called while tokenizing the first token
		if (description === undefined) {
			if (currentToken !== undefined && start === currentToken.start) {
				description = descriptions.PARSER_CORE.UNEXPECTED(currentToken.type);
			} else {
				if (this.isEOF(start)) {
					description = descriptions.PARSER_CORE.UNEXPECTED_EOF;
				} else {
					const char = this.input[ob1Get0(start)];
					description = descriptions.PARSER_CORE.UNEXPECTED_CHARACTER(char);
				}
			}
		}

		const descriptionWithCategory: DiagnosticDescription = {
			...description,
			advice: description.advice ?? [],
			category: description.category ?? this.diagnosticCategory,
		};

		return {
			description: descriptionWithCategory,
			location,
		};
	}

	// Return an error to indicate a parser error, this must be thrown at the callsite for refinement
	unexpected(opts: ParserUnexpectedOptions = {}): DiagnosticsError {
		return createSingleDiagnosticError(this.createDiagnostic(opts));
	}

	unexpectedDiagnostic(opts: ParserUnexpectedOptions = {}) {
		this.addDiagnostic(this.createDiagnostic(opts));
	}

	getDiagnosticLocation(
		opts: Omit<ParserUnexpectedOptions, "description"> = {},
	): RequiredProps<DiagnosticLocation, "start" | "end"> {
		let {start, end, token} = opts;

		if (opts.index !== undefined) {
			start = this.getPositionFromIndex(opts.index);
			end = start;
		}

		if (opts.startIndex !== undefined) {
			start = this.getPositionFromIndex(opts.startIndex);
		}

		if (opts.endIndex !== undefined) {
			end = this.getPositionFromIndex(opts.endIndex);
		}

		if (opts.location !== undefined) {
			start = opts.location.start;
			end = opts.location.end;
		}

		if (token !== undefined) {
			start = this.getPositionFromIndex(token.start);
			end = this.getPositionFromIndex(token.end);
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

		if (start === undefined || end === undefined) {
			throw new Error("This condition should have been refined");
		}

		let sourceText;
		if (this.path === undefined) {
			sourceText = this.sourceText;
		}

		return {
			sourceText,
			mtime: this.mtime,
			start,
			end,
			filename: this.filename,
		};
	}

	//# Token utility methods
	assertNoSpace(): void {
		if (this.currentToken.start !== this.prevToken.end) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.EXPECTED_SPACE,
			});
		}
	}

	// If the current token is the specified type then return the next token, otherwise return null
	eatToken(type: keyof Tokens): undefined | TokenValues<Tokens> {
		if (this.matchToken(type)) {
			return this.nextToken();
		} else {
			return undefined;
		}
	}

	didEatToken(type: keyof Tokens): boolean {
		return this.eatToken(type) !== undefined;
	}

	// Check if we're at the end of the input
	isEOF(index: Number0): boolean {
		return ob1Get0(index) >= this.input.length;
	}

	// Check if the current token matches the input type
	matchToken(type: keyof Tokens): boolean {
		return this.getToken().type === type;
	}

	// Get the current token and assert that it's of the specified type, the token stream will also be advanced
	expectToken<Type extends keyof Tokens>(
		type: Type,
		_metadata?: DiagnosticDescription,
	): Tokens[Type] {
		const token = this.getToken();
		if (token.type === type) {
			this.nextToken();
			// @ts-ignore
			return token;
		} else {
			throw this.unexpected({
				description: _metadata === undefined
					? descriptions.PARSER_CORE.EXPECTED_TOKEN(
							token.type,
							(type as string),
						)
					: _metadata,
			});
		}
	}

	getInputRange(
		start: Number0,
		count: number,
		startOffset?: number,
	): [string, Number0] {
		// Allow passing in an `offset` to avoid callsites having to do `ob1Add` themselves
		const startIndex = ob1Get0(
			startOffset === undefined ? start : ob1Add(start, startOffset),
		);
		const endIndex = Math.min(startIndex + count, this.input.length - 1);
		return [this.input.slice(startIndex, endIndex), ob1Coerce0(endIndex + 1)];
	}

	getInputCharOnly(index: Number0, offset?: number): string {
		return this.getInputChar(index, offset)[0];
	}

	getInputChar(index: Number0, offset?: number): [string, Number0] {
		const {input} = this;

		// Allow passing in an `offset` to avoid callsites having to do `ob1Add` themselves
		const i = ob1Get0(offset === undefined ? index : ob1Add(index, offset));

		const end = ob1Coerce0(i + 1);

		// Allow an overflow since we call this method to check for trailing characters
		if (i >= input.length || i < 0) {
			return ["", end];
		}

		return [input[i], end];
	}

	// Read from the input starting at the specified index, until the callback returns false
	readInputFrom(
		index: Number0,
		callback?: (char: string, index: Number0, input: string) => boolean,
	): [string, Number0, boolean] {
		const {input} = this;
		let value = "";

		while (true) {
			if (ob1Get0(index) >= input.length) {
				return [value, index, true];
			}

			if (
				callback === undefined ||
				callback(input[ob1Get0(index)], index, input)
			) {
				value += input[ob1Get0(index)];
				index = ob1Inc(index);
			} else {
				break;
			}
		}

		return [value, index, false];
	}

	// Get the string between the specified range
	getRawInput(start: Number0, end: Number0): string {
		return this.input.slice(ob1Get0(start), ob1Get0(end));
	}

	getInputStartIndex(node: undefined | NodeBase): Number0 {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.start, loc.filename);
	}

	getInputEndIndex(node: undefined | NodeBase): Number0 {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.end, loc.filename);
	}

	getLoc(node: undefined | NodeBase): SourceLocation {
		if (node === undefined || node.loc === undefined) {
			throw new Error("Tried to fetch node loc start but none found");
		} else {
			return node.loc;
		}
	}

	//# Token finalization

	finishToken<Type extends string>(
		type: Type,
		end: Number0 = ob1Inc(this.nextTokenIndex),
	): SimpleToken<Type> {
		return {
			type,
			start: this.nextTokenIndex,
			end,
		};
	}

	finishValueToken<Type extends string, Value>(
		type: Type,
		value: Value,
		end: Number0 = ob1Inc(this.nextTokenIndex),
	): ValueToken<Type, Value> {
		return {
			type,
			value,
			start: this.nextTokenIndex,
			end,
		};
	}

	finishComplexToken<Type extends string, Data>(
		type: Type,
		data: Data,
		end: Number0 = ob1Inc(this.nextTokenIndex),
	): ComplexToken<Type, Data> {
		return {
			type,
			...data,
			start: this.nextTokenIndex,
			end,
		};
	}

	//# SourceLocation finalization

	finishLocFromToken(token: TokenBase): SourceLocation {
		return this.finishLocAt(
			this.getPositionFromIndex(token.start),
			this.getPositionFromIndex(token.end),
		);
	}

	finishLoc(start: Position): SourceLocation {
		return this.finishLocAt(start, this.getLastEndPosition());
	}

	finishLocAt(start: Position, end: Position): SourceLocation {
		return {
			filename: this.filename,
			start,
			end,
		};
	}

	//# Node finalization

	finalizeNode<T extends AnyNode>(node: T): T {
		// @ts-ignore
		attachComments(this, node);
		return node;
	}

	// Sometimes we want to pretend we're in different locations to consume the comments of other nodes
	finishNodeWithStarts<T extends AnyNode>(starts: Array<Position>, node: T): T {
		for (const start of starts) {
			node = this.finishNode(start, node);
		}
		return node;
	}

	finishNode<T extends AnyNode>(
		start: Position,
		node: T,
	): T & {
		loc: SourceLocation;
	} {
		return this.finishNodeAt(start, this.getLastEndPosition(), node);
	}

	finishNodeAt<T extends AnyNode>(
		start: Position,
		end: Position,
		node: T,
	): T & {
		loc: SourceLocation;
	} {
		// Maybe mutating `node` is better...?
		const newNode: T & {
			loc: SourceLocation;
		} = {
			...node,
			loc: this.finishLocAt(start, end),
		};
		return this.finalizeNode(newNode);
	}

	finishRoot<T extends object>(node: T): T & RootBase {
		return {
			...node,
			corrupt: this.state.corrupt,
			mtime: this.mtime,
			diagnostics: this.getDiagnostics(),
			filename: this.getFilenameAssert(),
			comments: this.state.comments,
		};
	}

	finalize(): void {
		if (!this.eatToken("EOF")) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.EXPECTED_EOF,
			});
		}
	}

	//# Diagnostics

	getDiagnostics(): Diagnostics {
		const collector = new DiagnosticsProcessor({
			origins: [
				{
					category: this.diagnosticCategory,
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

	addDiagnostic(diag: Diagnostic) {
		this.state.diagnostics.push(diag);
	}

	addDiagnosticFilter(diag: DiagnosticFilter) {
		this.state.diagnosticFilters.push(diag);
	}

	addCompleteDiagnostic(diags: Diagnostics) {
		this.state.diagnostics = [...this.state.diagnostics, ...diags];
	}

	//# Comments

	registerComment(comment: AnyComment) {
		this.state.comments.push(comment);
		this.state.trailingComments.push(comment);
		this.state.leadingComments.push(comment);
	}
}

export class ParserWithRequiredPath<
	Tokens extends TokensShape,
	State extends ParserCoreState = ParserCoreState
> extends ParserCore<Tokens, State> {
	constructor(
		opts: ParserOptionsWithRequiredPath,
		diagnosticCategory: DiagnosticCategory,
		initialState: Omit<State, keyof ParserCoreState>,
	) {
		super(opts, diagnosticCategory, initialState);
		this.filename = this.getFilenameAssert();
		this.path = this.getPathAssert();
	}

	path: UnknownFilePath;
	filename: string;
}

const SOF_TOKEN: SOFToken = {
	type: "SOF",
	start: ob1Number0,
	end: ob1Number0,
};

type ParserSnapshot<Tokens extends TokensShape, State> = {
	nextTokenIndex: Number0;
	currentToken: TokenValues<Tokens>;
	prevToken: TokenValues<Tokens>;
	state: State;
};

function normalizeInput(opts: ParserOptions): string {
	const {input} = opts;

	if (input === undefined) {
		return "";
	} else if (opts.retainCarriageReturn) {
		return input;
	} else {
		return removeCarriageReturn(input);
	}
}
