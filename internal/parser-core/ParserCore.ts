import {
	ComplexToken,
	EOFToken,
	NodeBase,
	ParserCoreImplementation,
	ParserCoreTokenizeState,
	ParserCoreTypes,
	ParserUnexpectedOptions,
	Position,
	SOFToken,
	SimpleToken,
	SourceLocation,
	TokenBase,
	TokenValues,
	ValueToken,
} from "./types";
import {
	Diagnostic,
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
import {UnknownPath, createUnknownPath} from "@internal/path";
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

export default class ParserCore<Types extends ParserCoreTypes> {
	constructor(
		impl: ParserCoreImplementation<Types>,
		opts: Types["options"],
		meta: Types["meta"],
	) {
		const {
			path,
			mtime,
			offsetPosition,
			sourceText,
		} = opts;

		let {input} = opts;
		if (input === undefined) {
			input = "";
		} else if (!impl.retainCarriageReturn) {
			input = removeCarriageReturn(input);
		}
		if (impl.normalizeInput !== undefined) {
			input = impl.normalizeInput(input);
		}

		this.options = opts;
		this.meta = meta;
		this.impl = impl;

		// Input information
		this.path = path === undefined ? undefined : createUnknownPath(path);
		this.filename = this.path === undefined ? undefined : this.path.join();
		this.mtime = mtime;
		this.input = input;
		this.sourceText = sourceText ?? this.input;
		this.length = ob1Coerce0(this.input.length);

		this.eofToken = {
			type: "EOF",
			start: ob1Coerce0(this.input.length),
			end: ob1Coerce0(this.input.length),
		};

		// Parser/tokenizer state
		this.tokenizing = false;
		this.currLine =
			offsetPosition === undefined ? ob1Number1 : offsetPosition.line;
		this.currColumn =
			offsetPosition === undefined ? ob1Number0 : offsetPosition.column;
		this.nextTokenIndex = ob1Number0;
		this.currentToken = SOF_TOKEN;
		this.prevToken = SOF_TOKEN;
		this.comments = new CommentsConsumer();

		this.indexTracker = new PositionTracker({
			filename: this.filename,
			input: this.input,
			offsetPosition,
			getPosition: this.getPosition.bind(this),
		});

		let initialState: undefined | Types["state"];
		if (initialState === undefined && impl.getInitialState !== undefined) {
			initialState = impl.getInitialState(this);
		}

		// @ts-ignore
		this.state = {
			...initialState,
			...ParserCore.createInitialState(),
		};
	}

	public options: Types["options"];
	public meta: Types["meta"];
	public comments: CommentsConsumer;
	public indexTracker: PositionTracker;
	private impl: ParserCoreImplementation<Types>;
	private tokenizing: boolean;
	private nextTokenIndex: Number0;
	public state: Types["state"] & ParserCoreState;
	private prevToken: TokenValues<Types["tokens"]>;
	private currentToken: TokenValues<Types["tokens"]>;
	private eofToken: EOFToken;
	public path: undefined | UnknownPath;
	public filename: undefined | string;
	public input: string;
	public mtime: undefined | number;
	private sourceText: string;
	public length: Number0;
	private currLine: Number1;
	private currColumn: Number0;

	public static createInitialState(): ParserCoreState {
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

	public getPathAssert(): UnknownPath {
		const {path} = this;
		if (path === undefined) {
			throw new Error("Path expected but none was passed to this Parser");
		} else {
			return path;
		}
	}

	public getFilenameAssert(): string {
		const {filename} = this;
		if (filename === undefined) {
			throw new Error("Filename expected but none was passed to this Parser");
		} else {
			return filename;
		}
	}

	// Run the tokenizer over all tokens
	public tokenizeAll(): Array<TokenValues<Types["tokens"]>> {
		const tokens: Array<TokenValues<Types["tokens"]>> = [];

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
	public tokenize(index: Number0): undefined | TokenValues<Types["tokens"]> {
		const {tokenize} = this.impl;
		if (tokenize === undefined) {
			throw new Error("No tokenize implementation defined");
		} else {
			return tokenize(this, index);
		}
	}

	// Alternate tokenize method to allow that allows the use of state
	public tokenizeWithState(
		index: Number0,
		state: Types["state"],
	): undefined | ParserCoreTokenizeState<Types> {
		const {tokenizeWithState} = this.impl;
		if (tokenizeWithState !== undefined) {
			return tokenizeWithState(this, index, state);
		}

		const token = this.tokenize(index);
		if (token !== undefined) {
			return {token, state};
		}

		return undefined;
	}

	private _tokenizeWithState(
		index: Number0,
		state: Types["state"],
	): undefined | ParserCoreTokenizeState<Types> {
		if (this.impl.ignoreWhitespaceTokens) {
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
	public getToken(): TokenValues<Types["tokens"]> {
		const {currentToken} = this;
		if (currentToken === SOF_TOKEN) {
			return this.nextToken();
		} else {
			return currentToken;
		}
	}

	public save(): ParserSnapshot<Types> {
		return {
			nextTokenIndex: this.nextTokenIndex,
			currentToken: this.currentToken,
			prevToken: this.prevToken,
			state: this.state,
		};
	}

	public restore(snapshot: ParserSnapshot<Types>) {
		this.nextTokenIndex = snapshot.nextTokenIndex;
		this.currentToken = snapshot.currentToken;
		this.prevToken = snapshot.prevToken;
		this.state = snapshot.state;
	}

	// Advance to the next token, returning the new one
	public nextToken(): TokenValues<Types["tokens"]> {
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
	public getIndex(): Number0 {
		const {overrides} = this.impl;
		if (overrides !== undefined) {
			return overrides.getIndex(this);
		}

		return this.currentToken.start;
	}

	// Get the position of the current token
	public getPosition(): Position {
		const {overrides} = this.impl;
		if (overrides !== undefined) {
			return overrides.getPosition(this);
		}

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
	public getLastEndPosition(): Position {
		const {overrides} = this.impl;
		if (overrides !== undefined) {
			return overrides.getLastEndPosition(this);
		}

		return this.getPositionFromIndex(this.prevToken.end);
	}

	// Return the token that's after this current token without advancing to it
	public lookaheadToken(index?: Number0): TokenValues<Types["tokens"]> {
		return this.lookahead(index).token;
	}

	// Return the token and state that's after the current token without advancing to it
	public lookahead(
		index: Number0 = this.nextTokenIndex,
	): {
		token: TokenValues<Types["tokens"]>;
		state: ParserCoreState & Types["state"];
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
		const beforeState = this.state;
		const next = this._tokenizeWithState(index, beforeState);
		if (next === undefined) {
			throw this.unexpected({
				start: this.getPositionFromIndex(index),
			});
		}

		// Reset to old values
		this.tokenizing = wasTokenizing;
		this.nextTokenIndex = prevNextTokenIndex;

		return {
			token: next.token,
			state: {
				...beforeState,
				...next.state,
			},
		};
	}

	public getPositionFromIndex(index: Number0): Position {
		return this.indexTracker.getPositionFromIndex(index);
	}

	public getIndexFromPosition(
		pos: Position,
		filename: undefined | string,
	): Number0 {
		return this.indexTracker.getIndexFromPosition(pos, filename);
	}

	public createDiagnostic(opts: ParserUnexpectedOptions = {}): Diagnostic {
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
			category: description.category ?? this.impl.diagnosticCategory,
		};

		return {
			description: descriptionWithCategory,
			location,
		};
	}

	// Return an error to indicate a parser error, this must be thrown at the callsite for refinement
	public unexpected(opts: ParserUnexpectedOptions = {}): DiagnosticsError {
		return createSingleDiagnosticError(this.createDiagnostic(opts));
	}

	public unexpectedDiagnostic(opts: ParserUnexpectedOptions = {}) {
		this.addDiagnostic(this.createDiagnostic(opts));
	}

	public getDiagnosticLocation(
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
	public assertNoSpace(): void {
		if (this.currentToken.start !== this.prevToken.end) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.EXPECTED_SPACE,
			});
		}
	}

	// If the current token is the specified type then return the next token, otherwise return null
	public eatToken(
		type: keyof Types["tokens"],
	): undefined | TokenValues<Types["tokens"]> {
		if (this.matchToken(type)) {
			return this.nextToken();
		} else {
			return undefined;
		}
	}

	// Check if we're at the end of the input
	public isEOF(index: Number0): boolean {
		return ob1Get0(index) >= this.input.length;
	}

	// Check if the current token matches the input type
	public matchToken(type: keyof Types["tokens"]): boolean {
		return this.getToken().type === type;
	}

	// Get the current token and assert that it's of the specified type, the token stream will also be advanced
	public expectToken<Type extends keyof Types["tokens"]>(
		type: Type,
		_metadata?: DiagnosticDescription,
	): Types["tokens"][Type] {
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

	public getInputRange(
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

	public getInputCharOnly(index: Number0, offset?: number): string {
		return this.getInputChar(index, offset)[0];
	}

	public getInputChar(index: Number0, offset?: number): [string, Number0] {
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
	public readInputFrom(
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
	public getRawInput(start: Number0, end: Number0): string {
		return this.input.slice(ob1Get0(start), ob1Get0(end));
	}

	public getInputStartIndex(node: undefined | NodeBase): Number0 {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.start, loc.filename);
	}

	public getInputEndIndex(node: undefined | NodeBase): Number0 {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.end, loc.filename);
	}

	public getLoc(node: undefined | NodeBase): SourceLocation {
		if (node === undefined || node.loc === undefined) {
			throw new Error("Tried to fetch node loc start but none found");
		} else {
			return node.loc;
		}
	}

	//# Token finalization

	public finishToken<Type extends string>(
		type: Type,
		end: Number0 = ob1Inc(this.nextTokenIndex),
	): SimpleToken<Type> {
		return {
			type,
			start: this.nextTokenIndex,
			end,
		};
	}

	public finishValueToken<Type extends string, Value>(
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

	public finishComplexToken<Type extends string, Data>(
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

	public finishLocFromToken(token: TokenBase): SourceLocation {
		return this.finishLocAt(
			this.getPositionFromIndex(token.start),
			this.getPositionFromIndex(token.end),
		);
	}

	public finishLoc(start: Position): SourceLocation {
		return this.finishLocAt(start, this.getLastEndPosition());
	}

	public finishLocAt(start: Position, end: Position): SourceLocation {
		return {
			filename: this.filename,
			start,
			end,
		};
	}

	//# Node finalization

	public finalizeNode<T extends AnyNode>(node: T): T {
		// @ts-ignore
		attachComments(this, node);
		return node;
	}

	// Sometimes we want to pretend we're in different locations to consume the comments of other nodes
	public finishNodeWithStarts<T extends AnyNode>(
		starts: Array<Position>,
		node: T,
	): T {
		for (const start of starts) {
			node = this.finishNode(start, node);
		}
		return node;
	}

	public finishNode<T extends AnyNode>(
		start: Position,
		node: T,
	): T & {
		loc: SourceLocation;
	} {
		return this.finishNodeAt(start, this.getLastEndPosition(), node);
	}

	public finishNodeAt<T extends AnyNode>(
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

	public finishRoot<T extends object>(node: T): T & RootBase {
		return {
			...node,
			corrupt: this.state.corrupt,
			mtime: this.mtime,
			diagnostics: this.getDiagnostics(),
			filename: this.getFilenameAssert(),
			comments: this.state.comments,
		};
	}

	public finalize(): void {
		if (!this.eatToken("EOF")) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.EXPECTED_EOF,
			});
		}
	}

	//# Diagnostics

	public getDiagnostics(): Diagnostics {
		const collector = new DiagnosticsProcessor({
			origins: [
				{
					category: this.impl.diagnosticCategory,
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

	public addDiagnostic(diag: Diagnostic) {
		this.state.diagnostics.push(diag);
	}

	public addDiagnosticFilter(diag: DiagnosticFilter) {
		this.state.diagnosticFilters.push(diag);
	}

	public addCompleteDiagnostic(diags: Diagnostics) {
		this.state.diagnostics = [...this.state.diagnostics, ...diags];
	}

	//# Comments

	public registerComment(comment: AnyComment) {
		this.state.comments.push(comment);
		this.state.trailingComments.push(comment);
		this.state.leadingComments.push(comment);
	}
}

const SOF_TOKEN: SOFToken = {
	type: "SOF",
	start: ob1Number0,
	end: ob1Number0,
};

type ParserSnapshot<Types extends ParserCoreTypes> = {
	nextTokenIndex: Number0;
	currentToken: TokenValues<Types["tokens"]>;
	prevToken: TokenValues<Types["tokens"]>;
	state: ParserCoreState & Types["state"];
};
