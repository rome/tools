/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Position,
  SourceLocation,
  TokensShape,
  BaseTokens,
  EOFToken,
  SOFToken,
  ValueToken,
  NodeBase,
  SimpleToken,
  TokenBase,
  ComplexToken,
} from './types';
import {
  getDiagnosticsFromError,
  createSingleDiagnosticError,
  Diagnostic,
  DiagnosticCategory,
  DiagnosticDescription,
  DiagnosticsError,
  createBlessedDiagnosticMessage,
  descriptions,
} from '@romejs/diagnostics';
import {
  Number1,
  Number0,
  number1,
  number0,
  inc,
  coerce0,
  get0,
  add,
  sub,
  dec,
} from '@romejs/ob1';
import {escapeMarkup} from '@romejs/string-markup';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';
import {Class, OptionalProps} from '@romejs/typescript-helpers';

export * from './types';

export type ParserOptions = {
  path?: string | UnknownFilePath;
  mtime?: number;
  input?: string;
  offsetPosition?: Position;
};

export type ParserOptionsWithRequiredPath = Omit<ParserOptions, 'path'> & {
  path: NonNullable<ParserOptions['path']>;
};

export type ParserUnexpectedOptions = {
  description?: OptionalProps<DiagnosticDescription, 'category'>;
  loc?: SourceLocation;
  start?: Position;
  end?: Position;
  token?: TokenBase;
};

export type TokenValues<Tokens extends TokensShape> =
  | Tokens[keyof Tokens]
  | BaseTokens[keyof BaseTokens];

export function tryParseWithOptionalOffsetPosition<
  Opts extends ParserOptions,
  Ret
>(parserOpts: Opts, opts: {
  getOffsetPosition: () => Position;
  parse: (opts: Opts) => Ret;
}): Ret {
  try {
    return opts.parse(parserOpts);
  } catch (err) {
    const diagnostics = getDiagnosticsFromError(err);
    if (diagnostics === undefined) {
      throw err;
    } else {
      opts.parse({
        ...parserOpts,
        offsetPosition: opts.getOffsetPosition(),
      });
      throw new Error('Expected error');
    }
  }
}

const SOF_TOKEN: SOFToken = {
  type: 'SOF',
  start: number0,
  end: number0,
};

type ParserSnapshot<Tokens extends TokensShape, State> = {
  nextTokenIndex: Number0;
  currentToken: TokenValues<Tokens>;
  prevToken: TokenValues<Tokens>;
  state: State;
};

export class ParserCore<Tokens extends TokensShape, State> {
  constructor(
    opts: ParserOptions,
    diagnosticCategory: DiagnosticCategory,
    initialState: State,
  ) {
    const {path, mtime, input, offsetPosition} = opts;

    // Input information
    this.path = path === undefined ? undefined : createUnknownFilePath(path);
    this.filename = this.path === undefined ? undefined : this.path.join();
    this.mtime = mtime;
    this.input = input === undefined ? '' : input;
    this.length = coerce0(this.input.length);

    this.eofToken = {
      type: 'EOF',
      start: coerce0(this.input.length),
      end: coerce0(this.input.length),
    };

    // Parser/tokenizer state
    this.offsetPosition = offsetPosition;
    this.diagnosticCategory = diagnosticCategory;
    this.tokenizing = false;
    this.currLine = offsetPosition === undefined ? number1 : offsetPosition.line;
    this.currColumn = offsetPosition === undefined
      ? number0
      : offsetPosition.column;
    this.offsetIndex = offsetPosition === undefined
      ? number0
      : offsetPosition.index;
    this.startLine = this.currLine;
    this.startColumn = this.currColumn;
    this.nextTokenIndex = number0;
    this.currentToken = SOF_TOKEN;
    this.prevToken = SOF_TOKEN;
    this.state = initialState;
    this.ignoreWhitespaceTokens = false;

    this.latestPosition = {
      index: number0, // TODO this.offsetIndex
      line: this.currLine,
      column: this.currColumn,
    };
    this.cachedPositions = new Map();
  }

  offsetPosition: undefined | Position;
  startLine: Number1;
  startColumn: Number0;
  offsetIndex: Number0;
  diagnosticCategory: DiagnosticCategory;
  tokenizing: boolean;
  nextTokenIndex: Number0;
  state: State;
  prevToken: TokenValues<Tokens>;
  currentToken: TokenValues<Tokens>;
  latestPosition: Position;
  eofToken: EOFToken;
  ignoreWhitespaceTokens: boolean;
  cachedPositions: Map<Number0, Position>;

  path: undefined | UnknownFilePath;
  filename: undefined | string;

  mtime: undefined | number;
  input: string;
  length: Number0;
  currLine: Number1;
  currColumn: Number0;

  getPathAssert(): UnknownFilePath {
    const {path} = this;
    if (path === undefined) {
      throw new Error('Path expected but none was passed to this Parser');
    } else {
      return path;
    }
  }

  getFilenameAssert(): string {
    const {filename} = this;
    if (filename === undefined) {
      throw new Error('Filename expected but none was passed to this Parser');
    } else {
      return filename;
    }
  }

  // Run the tokenizer over all tokens
  tokenizeAll(): Array<TokenValues<Tokens>> {
    const tokens: Array<TokenValues<Tokens>> = [];
    try {
      while (!this.matchToken('EOF')) {
        tokens.push(this.getToken());
        this.nextToken();
      }
    } catch (err) {
      const diagnostics = getDiagnosticsFromError(err);
      if (diagnostics === undefined) {
        throw err;
      } else {
        tokens.push({
          type: 'Invalid',
          start: this.nextTokenIndex,
          end: this.length,
        });
      }
    }
    return tokens;
  }

  // Tokenize method that must be implemented by subclasses
  tokenize(index: Number0, input: string): undefined | TokenValues<Tokens> {
    throw new Error('Unimplemented');
  }

  // Alternate tokenize method to allow that allows the use of state
  tokenizeWithState(index: Number0, input: string, state: State): undefined | {
    token: TokenValues<Tokens>;
    state: State;
  } {
    const token = this.tokenize(index, input);
    if (token !== undefined) {
      return {token, state};
    } else {
      return undefined;
    }
  }

  _tokenizeWithState(index: Number0, input: string, state: State): undefined | {
    token: TokenValues<Tokens>;
    state: State;
  } {
    if (this.ignoreWhitespaceTokens) {
      switch (input[get0(index)]) {
        case ' ':
        case '\t':
        case '\r':
        case '\n':
          return this.lookahead(inc(index));
      }
    }

    return this.tokenizeWithState(index, input, state);
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

  // Get the position of the current token
  getPosition(): Position {
    const index = this.currentToken.start;

    const cached = this.cachedPositions.get(index);
    if (cached !== undefined) {
      return cached;
    }

    const pos: Position = {
      index: this.addOffset(index),
      line: this.currLine,
      column: this.currColumn,
    };
    this.cachedPositions.set(index, pos);
    return pos;
  }

  // Get the end position of the current token
  getLastEndPosition(): Position {
    return this.getPositionFromIndex(this.prevToken.end);
  }

  // Return the token that's after this current token without advancing to it
  lookaheadToken(index?: Number0): TokenValues<Tokens> {
    return this.lookahead(index).token;
  }

  // Return the token and state that's after the current token without advancing to it
  lookahead(index: Number0 = this.nextTokenIndex): {
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
    const nextToken = this._tokenizeWithState(index, this.input, this.state);
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

  addOffset(index: Number0): Number0 {
    return add(index, this.offsetIndex);
  }

  removeOffset(index: Number0): Number0 {
    return sub(index, this.offsetIndex);
  }

  getPositionFromIndex(index: Number0): Position {
    const cached = this.cachedPositions.get(index);
    if (cached !== undefined) {
      return cached;
    }

    let line: Number1 = number1;
    let column: Number0 = number0;
    let indexSearchOffset: number = 0;

    const indexWithOffset = this.addOffset(index);

    // Reuse existing line information if possible
    const {latestPosition} = this;
    const currPosition = this.getPosition();
    if (currPosition.index > latestPosition.index && currPosition.index <
        indexWithOffset) {
      line = currPosition.line;
      column = currPosition.column;
      indexSearchOffset = get0(this.removeOffset(currPosition.index));
    } else if (latestPosition.index < indexWithOffset) {
      line = latestPosition.line;
      column = latestPosition.column;
      indexSearchOffset = get0(this.removeOffset(latestPosition.index));
    }

    // Read the rest of the input until we hit the index
    for (let i = indexSearchOffset; i < get0(index); i++) {
      const char = this.input[i];

      if (char === '\n') {
        line = inc(line);
        column = number0;
      } else {
        column = inc(column);
      }
    }

    const pos: Position = {
      index: indexWithOffset,
      line,
      column,
    };

    if (latestPosition === undefined || pos.index > latestPosition.index) {
      this.latestPosition = pos;
    }

    this.cachedPositions.set(index, pos);
    return pos;
  }

  createDiagnostic(opts: ParserUnexpectedOptions = {}): Diagnostic {
    const {currentToken} = this;
    let {description: metadata, start, end, loc, token} = opts;

    // Allow passing in a TokenBase
    if (token !== undefined) {
      start = this.getPositionFromIndex(token.start);
      end = this.getPositionFromIndex(token.end);
    }

    // Allow passing in a SourceLocation as an easy way to point to a particular node
    if (loc !== undefined) {
      start = loc.start;
      end = loc.end;
    }

    // When both properties are omitted then we will default to the current token range
    if (start === undefined && end === undefined) {
      end = this.getLastEndPosition();
    }

    if (start === undefined) {
      start = this.getPosition();
    }

    if (end === undefined) {
      end = start;
    }

    // Sometimes the end position may be empty as it hasn't been filled yet
    if (end.index === number0) {
      end = start;
    }

    // Normalize message, we need to be defensive here because it could have been called while tokenizing the first token
    if (metadata === undefined) {
      let message;
      if (currentToken !== undefined && start !== undefined && start.index ===
          currentToken.start) {
        message = createBlessedDiagnosticMessage(
          `Unexpected ${currentToken.type}`,
        );
      } else {
        if (this.isEOF(start.index)) {
          message = createBlessedDiagnosticMessage('Unexpected end of file');
        } else {
          const char = this.input[get0(start.index)];
          message = createBlessedDiagnosticMessage(
            `Unexpected character <emphasis>${escapeMarkup(char)}</emphasis>`,
          );
        }
      }
      metadata = {message};
    }

    const metadataWithCategory: DiagnosticDescription = {
      ...metadata,
      category: metadata.category === undefined
        ? this.diagnosticCategory
        : metadata.category,
    };

    return {
      description: metadataWithCategory,
      location: {
        sourceText: this.path === undefined ? this.input : undefined,
        mtime: this.mtime,
        start,
        end,
        filename: this.filename,
      },
    };
  }

  // Return an error to indicate a parser error, this must be thrown at the callsite for refinement
  unexpected(opts: ParserUnexpectedOptions = {}): DiagnosticsError {
    return createSingleDiagnosticError(this.createDiagnostic(opts));
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
    return get0(index) >= this.input.length;
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
      throw this.unexpected(
        {
          description: _metadata === undefined
            ? descriptions.PARSER_CORE.EXPECTED_TOKEN(
              token.type,
              (type as string),
            )
            : _metadata,
        },
      );
    }
  }

  // Read from the input starting at the specified index, until the callback returns false
  readInputFrom(
    index: Number0,
    callback?: (char: string, index: Number0, input: string) => boolean,
  ): [string, Number0, boolean] {
    const {input} = this;
    let value = '';

    while (true) {
      if (get0(index) >= input.length) {
        return [value, index, true];
      }

      if (callback === undefined || callback(input[get0(index)], index, input)) {
        value += input[get0(index)];
        index = inc(index);
      } else {
        break;
      }
    }

    return [value, index, false];
  }

  // Get the string between the specified range
  getRawInput(start: Number0, end: Number0): string {
    return this.input.slice(get0(start), get0(end));
  }

  //# Utility methods to make it easy to construct nodes or tokens
  getLoc(node: undefined | NodeBase): SourceLocation {
    if (node === undefined || node.loc === undefined) {
      throw new Error('Tried to fetch node loc start but none found');
    } else {
      return node.loc;
    }
  }

  finishToken<Type extends string>(
    type: Type,
    end: Number0 = inc(this.nextTokenIndex),
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
    end: Number0 = inc(this.nextTokenIndex),
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
    end: Number0 = inc(this.nextTokenIndex),
  ): ComplexToken<Type, Data> {
    return {
      type,
      ...data,
      start: this.nextTokenIndex,
      end,
    };
  }

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

  finalize(): void {
    if (!this.eatToken('EOF')) {
      throw this.unexpected({
        description: descriptions.PARSER_CORE.EXPECTED_EOF,
      });
    }
  }
}

export class ParserWithRequiredPath<Tokens extends TokensShape, State> extends ParserCore<
  Tokens,
  State
> {
  constructor(
    opts: ParserOptionsWithRequiredPath,
    diagnosticCategory: DiagnosticCategory,
    initialState: State,
  ) {
    super(opts, diagnosticCategory, initialState);
    this.filename = this.getFilenameAssert();
    this.path = this.getPathAssert();
  }

  path: UnknownFilePath;
  filename: string;
}

//# Helpers methods for basic token parsing
export function isDigit(char: undefined | string): boolean {
  return char !== undefined && /[0-9]/.test(char);
}

export function isAlpha(char: undefined | string): boolean {
  return char !== undefined && /[A-Za-z]/.test(char);
}

export function isHexDigit(char: undefined | string): boolean {
  return char !== undefined && /[0-9A-Fa-f]/.test(char);
}

export function isESIdentifierChar(char: undefined | string): boolean {
  return char !== undefined && /[A-F0-9a-z_$]/.test(char);
}

export function isESIdentifierStart(char: undefined | string): boolean {
  return char !== undefined && /[A-Fa-z_$]/.test(char);
}

export function isEscaped(index: Number0, input: string): boolean {
  const prevChar = input[get0(index) - 1];

  if (prevChar === '\\') {
    return !isEscaped(dec(index), input);
  } else {
    return false;
  }
}

export function readUntilLineBreak(char: string): boolean {
  return char !== '\n';
}

// Lazy initialize a ParserCore subclass... Circular dependencies are wild and necessitate this as ParserCore may not be available
export function createParser<
  T,
  Args extends Array<unknown>
>(
  callback: (
    parser: typeof ParserCore,
    parserRequiredPath: typeof ParserWithRequiredPath,
  ) => Class<T, Args>,
): (...args: Args) => T {
  let klass: undefined | Class<T, Args>;

  return (...args: Args) => {
    if (klass === undefined) {
      klass = callback(ParserCore, ParserWithRequiredPath);
    }

    return new klass(...args);
  };
}

// Utility methods for dealing with nodes
export function extractSourceLocationRangeFromNodes(
  nodes: Array<{loc?: SourceLocation}>,
): undefined | SourceLocation {
  if (nodes.length === 0) {
    return undefined;
  }

  let filename: undefined | string = undefined;
  let start: undefined | Position = undefined;
  let end: undefined | Position = undefined;

  for (const node of nodes) {
    const {loc} = node;
    if (loc === undefined) {
      continue;
    }

    if (start === undefined || loc.start.index < start.index) {
      start = loc.start;
    }

    if (end === undefined || loc.end.index > end.index) {
      end = loc.end;
    }

    if (filename === undefined) {
      filename = loc.filename;
    } else if (filename !== loc.filename) {
      throw new Error(
        `Mixed filenames in node, expected ${filename} but got ${loc.filename}`,
      );
    }
  }

  if (start === undefined || end === undefined) {
    return undefined;
  }

  return {
    filename,
    start,
    end,
  };
}
