import {
	EOFToken,
	NodeBase,
	ParserCoreImplementation,
	ParserCoreOverrides,
	ParserCoreTokenizeState,
	ParserCoreTypes,
	ParserUnexpectedOptions,
	Position,
	SOFToken,
	SourceLocation,
	TokenBase,
	TokenValues,
} from "./types";
import {
	DIAGNOSTIC_CATEGORIES,
	Diagnostic,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticDescriptionOptional,
	DiagnosticEliminationFilter,
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticsError,
	DiagnosticsProcessor,
	catchDiagnosticsSync,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {AnyComment, AnyNode, RootBase} from "@internal/ast";
import {Path, UNKNOWN_PATH, equalPaths} from "@internal/path";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {CommentsConsumer} from "@internal/js-parser";
import PositionTracker from "./PositionTracker";
import {RequiredProps} from "@internal/typescript-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {attachComments} from "./comments";
import {pretty} from "@internal/pretty-format";
import TokenizerCore from "./TokenizerCore";

export type ParserCoreState = {
	comments: AnyComment[];
	trailingComments: AnyComment[];
	leadingComments: AnyComment[];
	commentStack: AnyNode[];
	commentPreviousNode: undefined | AnyNode;
	diagnostics: Diagnostic[];
	diagnosticFilters: DiagnosticEliminationFilter[];
	corrupt: boolean;
};

type TokenNames<Types extends ParserCoreTypes> = keyof Types["tokens"];

export default class ParserCore<Types extends ParserCoreTypes> {
	constructor(
		impl: ParserCoreImplementation<Types>,
		opts: Types["options"],
		meta: Types["meta"],
		overrides: ParserCoreOverrides = {},
	) {
		const {
			path,
			integrity,
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
		this.language = overrides.diagnosticLanguage ?? impl.diagnosticLanguage;
		this.diagnosticCategory =
			overrides.diagnosticCategory ??
			impl.diagnosticCategory ??
			DIAGNOSTIC_CATEGORIES.parse;
		this.diagnosticCategoryValue =
			overrides.diagnosticCategoryValue ??
			impl.diagnosticCategoryValue ??
			this.language;

		// Input information
		this.path = path ?? UNKNOWN_PATH;
		this.integrity = integrity;
		this.input = input;
		this.sourceText = sourceText ?? this.input;
		this.length = this.input.length;

		this.eofToken = {
			type: "EOF",
			start: new ZeroIndexed(this.input.length),
			end: new ZeroIndexed(this.input.length),
		};

		// Parser/tokenizer state
		this.tokenizing = false;

		const indexTracker = new PositionTracker({
			path: this.path,
			input: this.input,
			offsetPosition,
			getPosition: this.getPosition.bind(this),
		});

		this.indexTracker = indexTracker;

		this.tokenizer = new TokenizerCore({
			input: this.input,
			indexTracker,
			parser: this,
		});

		this.reset();
	}

	public options: Types["options"];
	public meta: Types["meta"];
	public tokenizer: TokenizerCore<Types>;
	public indexTracker: PositionTracker;
	public impl: ParserCoreImplementation<Types>;
	private tokenizing: boolean;
	private eofToken: EOFToken;
	public path: Path;
	public input: string;
	public language: DiagnosticLanguage;
	public integrity: undefined | DiagnosticIntegrity;
	private sourceText: string;
	public length: number;
	private diagnosticCategory: DiagnosticCategory;
	private diagnosticCategoryValue: string;

	private cachedDiagnostics:
		| undefined
		| {
				diagnostics: Diagnostic[];
				filters: DiagnosticEliminationFilter[];
				rawDiagnostics: Diagnostic[];
			};

	// Internal mutable state
	public comments!: CommentsConsumer;
	private nextTokenIndex!: ZeroIndexed;
	public state!: Types["state"] & ParserCoreState;
	private prevToken!: TokenValues<Types["tokens"]>;
	private currentToken!: TokenValues<Types["tokens"]>;
	private currLine!: OneIndexed;
	private currColumn!: ZeroIndexed;

	public getInputCharOnly(index: ZeroIndexed): string {
		return this.input[index.valueOf()] ?? "";
	}

	// Reset the parser and it's initial positions to the initial state
	public reset() {
		const {offsetPosition} = this.options;
		const {impl} = this;

		this.currLine =
			offsetPosition === undefined ? new OneIndexed() : offsetPosition.line;
		this.currColumn =
			offsetPosition === undefined ? new ZeroIndexed() : offsetPosition.column;
		this.nextTokenIndex = new ZeroIndexed();

		this.comments = new CommentsConsumer();

		const sofToken: SOFToken = {
			type: "SOF",
			start: new ZeroIndexed(),
			end: new ZeroIndexed(),
		};
		this.currentToken = sofToken;
		this.prevToken = sofToken;

		let initialState: undefined | Types["state"];
		if (initialState === undefined && impl.getInitialState !== undefined) {
			initialState = impl.getInitialState(this);
		}

		this.state = {
			...initialState,
			...ParserCore.createInitialState(),
		};
	}

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

	// Run the tokenizer over all tokens
	public getAllTokens(): TokenValues<Types["tokens"]>[] {
		const tokens: TokenValues<Types["tokens"]>[] = [];

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
				end: new ZeroIndexed(this.length),
			});
		}

		return tokens;
	}

	// Alternate tokenize method to allow that allows the use of state
	public tokenize(
		index: ZeroIndexed,
		state: Types["state"],
	): undefined | ParserCoreTokenizeState<Types> {
		const {tokenizer} = this;
		tokenizer.setTokenStart(index);

		if (this.impl.ignoreWhitespaceTokens) {
			switch (tokenizer.get()) {
				case " ":
				case "\t":
				case "\n":
					return this.lookahead(index.increment());

				case "\r": {
					if (tokenizer.eat("\r\n")) {
						return this.lookahead(index.add(2));
					}
					break;
				}
			}
		}

		const {tokenizeWithState, tokenize} = this.impl;
		if (tokenizeWithState !== undefined) {
			return tokenizeWithState(this, tokenizer, state);
		}

		if (tokenize === undefined) {
			throw new Error("No tokenize or tokenizeWithState implementation defined");
		}

		const token = tokenize(this, tokenizer);
		if (token !== undefined) {
			return token;
		}

		return undefined;
	}

	public getToken(): TokenValues<Types["tokens"]> {
		const {currentToken} = this;
		if (currentToken.type === "SOF") {
			return this.nextToken();
		} else {
			return currentToken;
		}
	}

	public getCurrentToken(): TokenValues<Types["tokens"]> {
		return this.currentToken;
	}

	public getPreviousToken(): TokenValues<Types["tokens"]> {
		return this.prevToken;
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
		const [state, nextToken] = this.lookahead();

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

	public setState(state: Partial<Types["state"]>): void {
		this.state = {
			...this.state,
			...state,
		};
	}

	// Get the start index of the current token
	public getIndex(): ZeroIndexed {
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
		const cached = this.indexTracker.cachedPositions.get(index.valueOf());
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
	public lookaheadToken(index?: ZeroIndexed): TokenValues<Types["tokens"]> {
		return this.lookahead(index)[1];
	}

	// Return the token and state that's after the current token without advancing to it
	public lookahead(
		index: ZeroIndexed = this.nextTokenIndex,
	): [Types["state"] & ParserCoreState, TokenValues<Types["tokens"]>] {
		if (this.isEOF(index)) {
			return [this.state, this.eofToken];
		}

		// Set the next token index, in the case of a lookahead we'll set it back later
		const prevNextTokenIndex = this.nextTokenIndex;
		this.nextTokenIndex = index;

		// Indicate that we're currently tokenizing to catch some weird recursive tokenizing errors
		const wasTokenizing = this.tokenizing;
		this.tokenizing = true;

		// Tokenize and do some validation
		const beforeState = this.state;
		let next = this.tokenize(index, beforeState);
		if (next === undefined) {
			throw this.unexpected({
				start: this.getPositionFromIndex(index),
			});
		}

		// Reset to old values
		this.tokenizing = wasTokenizing;
		this.nextTokenIndex = prevNextTokenIndex;

		if (Array.isArray(next)) {
			return [
				beforeState === next[0]
					? beforeState
					: {
							...beforeState,
							...next[0],
						},
				next[1],
			];
		} else {
			return [beforeState, next];
		}
	}

	public getPositionFromIndex(index: number | ZeroIndexed): Position {
		return this.indexTracker.getPositionFromIndex(index);
	}

	public getIndexFromPosition(pos: Position): ZeroIndexed {
		return this.indexTracker.getIndexFromPosition(pos, this.path);
	}

	public createDiagnostic(opts: ParserUnexpectedOptions = {}): Diagnostic {
		const {currentToken} = this;
		let {description} = opts;
		const location = this.getDiagnosticLocation(opts);

		// Normalize message, we need to be defensive here because it could have been called while tokenizing the first token
		if (description === undefined) {
			const start = this.getIndexFromPosition(location.start);
			const end = this.getIndexFromPosition(location.end);

			let tokenType;
			if (start.equal(currentToken?.start)) {
				tokenType = currentToken.type;
			}

			if (this.isEOF(start)) {
				description = descriptions.PARSER_CORE.UNEXPECTED_EOF;
			} else {
				const str = this.input.slice(start.valueOf(), end.valueOf());
				if (str.length === 1) {
					description = descriptions.PARSER_CORE.UNEXPECTED_CHARACTER(
						str,
						tokenType,
					);
				} else {
					description = descriptions.PARSER_CORE.UNEXPECTED_CHARACTERS(
						str,
						tokenType,
					);
				}
			}
		}

		const descriptionWithCategory: DiagnosticDescription = {
			...description,
			advice: description.advice ?? [],
			category: description.category ?? this.diagnosticCategory,
			categoryValue: description.category === undefined
				? this.diagnosticCategoryValue
				: description.categoryValue,
		};

		return {
			tags: this.impl.diagnosticTags,
			description: descriptionWithCategory,
			location,
		};
	}

	// Return an error to indicate a parser error, this must be thrown at the callsite for refinement
	public unexpected(opts: ParserUnexpectedOptions = {}): DiagnosticsError {
		return createSingleDiagnosticsError(this.createDiagnostic(opts));
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

		// If we weren't given a start, end, or token then point to the current token
		if (
			start === undefined &&
			end === undefined &&
			token === undefined &&
			this.currentToken.type !== "SOF"
		) {
			token = this.getToken();
		}

		if (token !== undefined) {
			start = this.getPositionFromIndex(token.start);
			end = this.getPositionFromIndex(token.end);
		}

		// If no start or end, just point to the current position
		if (start === undefined && end === undefined) {
			start = this.getPosition();
		}

		const loc = opts.loc ?? opts.node?.loc;
		if (start === undefined && end === undefined && loc !== undefined) {
			start = loc.start;
			end = loc.end;
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
		if (
			this.options.path === undefined ||
			this.options.includeSourceTextInDiagnostics
		) {
			sourceText = this.sourceText;
		}

		return {
			language: this.language,
			sourceText,
			integrity: this.integrity,
			start,
			end,
			path: this.path,
		};
	}

	//# Token utility methods
	public assertNoSpace(): void {
		if (!this.currentToken.start.equal(this.prevToken.end)) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.UNEXPECTED_SPACE,
				startIndex: this.prevToken.end,
				endIndex: this.currentToken.start,
			});
		}
	}

	public assertNoNewline(
		prevToken: TokenValues<Types["tokens"]> = this.prevToken,
	): void {
		const prevLine = this.getPositionFromIndex(prevToken.start).line;
		const currLine = this.getPositionFromIndex(this.currentToken.start).line;
		if (!currLine.equal(prevLine)) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.UNEXPECTED_NEWLINE,
				startIndex: prevToken.end,
				endIndex: this.currentToken.start,
			});
		}
	}

	public assertNewline(
		prevToken: TokenValues<Types["tokens"]> = this.prevToken,
	): void {
		const prevLine = this.getPositionFromIndex(prevToken.start).line;
		const currLine = this.getPositionFromIndex(this.currentToken.start).line;
		if (currLine.equal(prevLine)) {
			throw this.unexpected({
				description: descriptions.PARSER_CORE.EXPECTED_NEWLINE,
				startIndex: prevToken.end,
				endIndex: this.currentToken.start,
			});
		}
	}

	// If the current token is the specified type then return the next token, otherwise return null
	public eatToken<Type extends keyof Types["tokens"]>(
		type: Type,
	): undefined | Types["tokens"][Type] {
		const token = this.getToken();
		if (token.type === type) {
			this.nextToken();
			// @ts-expect-error
			return token;
		} else {
			return undefined;
		}
	}

	// Check if we're at the end of the input
	public isEOF(index: ZeroIndexed): boolean {
		return index.valueOf() >= this.input.length;
	}

	// Assert that the current token matches the input type
	public matchToken(type: TokenNames<Types>): boolean {
		return this.getToken().type === type;
	}

	// Check if the current token matches the input type
	public assertToken<Type extends TokenNames<Types>>(
		type: Type,
		_metadata?: DiagnosticDescriptionOptional,
	): Types["tokens"][Type] {
		const token = this.getToken();
		if (token.type === type) {
			// @ts-expect-error
			return token;
		} else {
			throw this.unexpected({
				description: _metadata === undefined
					? descriptions.PARSER_CORE.EXPECTED_TOKEN(token.type, type as string)
					: _metadata,
			});
		}
	}

	// Get the current token and assert that it's of the specified type, the token stream will also be advanced
	public expectToken<Type extends TokenNames<Types>>(
		type: Type,
		_metadata?: DiagnosticDescriptionOptional,
	): Types["tokens"][Type] {
		const token = this.assertToken(type, _metadata);
		this.nextToken();
		return token;
	}

	// Check if there was no gaps between the current token and the previous
	public areTokensInputSiblings(): boolean {
		return this.getPreviousToken().end === this.getToken().start;
	}

	public getInputRange(start: ZeroIndexed, count: number): [string, ZeroIndexed] {
		const sub = this.getInputRangeOnly(start, count);
		const end = new ZeroIndexed(start.valueOf() + sub.length + 1);
		return [sub, end];
	}

	public getInputRangeOnly(start: ZeroIndexed, count: number): string {
		return this.getRawInput(start, start.add(count));
	}

	// Get the string between the specified range
	public getRawInput(
		start: number | ZeroIndexed,
		end: number | ZeroIndexed,
	): string {
		return this.input.slice(start.valueOf(), end.valueOf());
	}

	public getInputStartIndex(node: undefined | NodeBase): ZeroIndexed {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.start);
	}

	public getInputEndIndex(node: undefined | NodeBase): ZeroIndexed {
		const loc = this.getLoc(node);
		return this.getIndexFromPosition(loc.end);
	}

	public getLoc(node: undefined | NodeBase): SourceLocation {
		if (node === undefined || node.loc === undefined) {
			throw new Error("Tried to fetch node loc start but none found");
		} else {
			const {loc} = node;
			if (!equalPaths(loc.path, this.path)) {
				throw new Error(
					pretty`Filename mismatch. SourceLocation ${loc.path} is different than the filename we're tracking of ${this.path}`,
				);
			}
			return loc;
		}
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
			path: this.path,
			start,
			end,
		};
	}

	//# Node finalization

	public finalizeNode<T extends AnyNode>(node: T): T {
		// @ts-expect-error
		attachComments(this, node);
		return node;
	}

	// Sometimes we want to pretend we're in different locations to consume the comments of other nodes
	public finishNodeWithStarts<T extends AnyNode>(starts: Position[], node: T): T {
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
			integrity: this.integrity,
			diagnostics: this.getDiagnostics(),
			path: this.path,
			comments: this.state.comments,
		};
	}

	public finalize(shouldThrow: boolean = true): void {
		if (!this.eatToken("EOF")) {
			if (shouldThrow) {
				this.unexpectedDiagnostic({
					description: descriptions.PARSER_CORE.EXPECTED_EOF,
				});
			} else {
				throw this.unexpected({
					description: descriptions.PARSER_CORE.EXPECTED_EOF,
				});
			}
		}

		if (shouldThrow) {
			const diagnostics = this.getDiagnostics();
			if (diagnostics.length > 0) {
				throw new DiagnosticsError("ParserCore#finalize", diagnostics);
			}
		}
	}

	//# Diagnostics

	public getDiagnostics(): Diagnostic[] {
		const {cachedDiagnostics, state} = this;
		if (
			cachedDiagnostics?.filters === state.diagnosticFilters &&
			cachedDiagnostics.rawDiagnostics === state.diagnostics
		) {
			return cachedDiagnostics.diagnostics;
		}

		const processor = new DiagnosticsProcessor({
			origin: {
				entity: `ParserCore<${this.language}>`,
			},
		});

		for (const filter of state.diagnosticFilters) {
			processor.addEliminationFilter(filter);
		}

		// TODO remove any trailing "eof" diagnostic
		processor.addDiagnostics(state.diagnostics);

		const diagnostics = processor.getDiagnostics().slice(0, 1);
		this.cachedDiagnostics = {
			filters: state.diagnosticFilters,
			rawDiagnostics: state.diagnostics,
			diagnostics,
		};
		return diagnostics;
	}

	public addDiagnostic(diag: Diagnostic) {
		this.state.diagnostics.push(diag);
	}

	public addDiagnosticFilter(diag: DiagnosticEliminationFilter) {
		this.state.diagnosticFilters.push(diag);
	}

	public addCompleteDiagnostic(diags: Diagnostic[]) {
		this.state.diagnostics = [...this.state.diagnostics, ...diags];
	}

	//# Comments

	public registerComment(comment: AnyComment) {
		this.state.comments.push(comment);
		this.state.trailingComments.push(comment);
		this.state.leadingComments.push(comment);
	}
}

type ParserSnapshot<Types extends ParserCoreTypes> = {
	nextTokenIndex: ZeroIndexed;
	currentToken: TokenValues<Types["tokens"]>;
	prevToken: TokenValues<Types["tokens"]>;
	state: ParserCoreState & Types["state"];
};
