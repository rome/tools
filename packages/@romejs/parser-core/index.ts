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
} from './types';
import {
  PartialDiagnosticAdvice,
  getDiagnosticsFromError,
  createSingleDiagnosticError,
} from '@romejs/diagnostics';
import {
  Number1,
  Number0,
  number1,
  number0,
  sub,
  inc,
  coerce0,
  number0Neg1,
  get0,
} from '@romejs/ob1';
import {escapeMarkup} from '@romejs/string-markup';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';
import {Class} from '@romejs/typescript-helpers';

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
  message?: string;
  loc?: SourceLocation;
  start?: Position;
  end?: Position;
  advice?: PartialDiagnosticAdvice;
};

export type TokenValues<Tokens extends TokensShape> =
  | Tokens[keyof Tokens]
  | BaseTokens[keyof BaseTokens];

export function tryParseWithOptionalOffsetPosition<
  Opts extends ParserOptions,
  Ret
>(
  parserOpts: Opts,
  opts: {
    getOffsetPosition: () => Position;
    parse: (opts: Opts) => Ret;
  },
): Ret {
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
  start: number0Neg1,
  end: number0Neg1,
};

export class ParserCore<Tokens extends TokensShape, State> {
  constructor(opts: ParserOptions, parserName: string, initialState: State) {
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
    this.parserName = parserName;
    this.tokenizing = false;
    this.currLine =
      offsetPosition === undefined ? number1 : offsetPosition.line;
    this.currColumn =
      offsetPosition === undefined ? number0 : offsetPosition.column;
    this.startLine = this.currLine;
    this.startColumn = this.currColumn;
    this.nextTokenIndex = number0;
    this.currentToken = SOF_TOKEN;
    this.prevToken = SOF_TOKEN;
    this.state = initialState;
    this.ignoreWhitespaceTokens = false;
  }

  offsetPosition: undefined | Position;
  startLine: Number1;
  startColumn: Number0;
  parserName: string;
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
  tokenizeWithState(
    index: Number0,
    input: string,
    state: State,
  ): undefined | {token: TokenValues<Tokens>; state: State} {
    const token = this.tokenize(index, input);
    if (token !== undefined) {
      return {token, state};
    }
  }

  _tokenizeWithState(
    index: Number0,
    input: string,
    state: State,
  ): undefined | {token: TokenValues<Tokens>; state: State} {
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

  // Advance to the next token, returning the new one
  nextToken(): TokenValues<Tokens> {
    if (this.isEOF()) {
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

    // Keep currLine and currColumn up to date
    for (let i = get0(prevToken.start); i < get0(nextToken.start); i++) {
      const char = this.input[i];
      if (char === '\n') {
        this.currLine = inc(this.currLine);
        this.currColumn = number0;
      } else {
        this.currColumn = inc(this.currColumn);
      }
    }

    this.nextTokenIndex = nextToken.end;
    this.prevToken = prevToken;
    this.currentToken = nextToken;
    this.state = state;
    return nextToken;
  }

  // Get the position of the current token
  getPosition(): Position {
    const {currentToken} = this;
    return {
      index: currentToken === undefined ? number0 : currentToken.start,
      line: this.currLine,
      column: this.currColumn,
    };
  }

  // Get the end position of the current token
  getEndPosition() {
    const token = this.getToken();
    return this.getPositionFromIndex(token.end);
  }

  // Get the end position of the previous token
  getPrevEndPosition() {
    return this.getPositionFromIndex(this.prevToken.end);
  }

  // Return the token that's after this current token without advancing to it
  lookaheadToken(index?: Number0): TokenValues<Tokens> {
    return this.lookahead(index).token;
  }

  // Return the token and state that's after the current token without advancing to it
  lookahead(
    index: Number0 = this.nextTokenIndex,
  ): {token: TokenValues<Tokens>; state: State} {
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

  getPositionFromIndex(index: Number0): Position {
    const targetIndex = index;

    // Find the line/column relative to the source
    let line: Number1 = this.startLine;
    let column: Number0 = this.startColumn;

    // Reuse existing line information if possible
    const currPosition = this.getPosition();
    if (currPosition.index < index) {
      line = currPosition.line;
      column = currPosition.column;
      index = sub(index, currPosition.index);
    }

    // Read the rest of the input until we hit the index
    for (let i = 0; i < get0(index); i++) {
      const char = this.input[i];

      if (char === '\n') {
        line = inc(line);
        column = number0;
      } else {
        column = inc(column);
      }
    }

    return {
      index: targetIndex,
      line,
      column,
    };
  }

  // Return an error to indicate a parser error, this must be thrown at the callsite for refinement
  unexpected(opts: ParserUnexpectedOptions = {}) {
    const {currentToken} = this;
    let {message, start, end, loc} = opts;

    // Allow passing in a SourceLocation as an easy way to point to a particular node
    if (loc !== undefined) {
      start = loc.start;
      end = loc.end;
    }

    // When both properties are omitted then we will default to the current token range
    if (start === undefined && end === undefined) {
      end = this.getEndPosition();
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
    if (message === undefined) {
      if (
        currentToken !== undefined &&
        start !== undefined &&
        start.index === currentToken.start
      ) {
        message = `Unexpected ${currentToken.type}`;
      } else {
        const char = this.input[get0(start.index)];
        message = `Unexpected character <emphasis>${escapeMarkup(
          char,
        )}</emphasis>`;
      }
    }

    let errMessage = `${message} (${start.line}:${start.column})`;
    if (this.path !== undefined) {
      errMessage = `${this.path}: ${errMessage} Input: ${this.input}`;
    }

    throw createSingleDiagnosticError({
      message,
      advice: opts.advice,
      category: this.parserName,
      sourceText: this.path === undefined ? this.input : undefined,
      mtime: this.mtime,
      start,
      end,
      filename: this.filename,
    });
  }

  //# Token utility methods

  assertNoSpace() {
    if (this.currentToken.start !== this.prevToken.end) {
      throw this.unexpected({
        message: 'Expected no space between',
      });
    }
  }

  // If the current token is the specified type then return the next token, otherwise return null
  eatToken(type: keyof Tokens): undefined | TokenValues<Tokens> {
    if (this.matchToken(type)) {
      return this.nextToken();
    }
  }

  // Check if we're at the end of the input
  isEOF(index: Number0 = this.nextTokenIndex): boolean {
    return get0(index) >= this.input.length;
  }

  // Check if the current token matches the input type
  matchToken(type: keyof Tokens): boolean {
    return this.getToken().type === type;
  }

  // Get the current token and assert that it's of the specified type, the token stream will also be advanced
  expectToken<Type extends keyof Tokens>(
    type: Type,
    message?: string,
  ): Tokens[Type] {
    const token = this.getToken();
    if (token.type === type) {
      this.nextToken();
      // @ts-ignore
      return token;
    } else {
      throw this.unexpected({
        message:
          message === undefined
            ? `Expected token ${type} but got ${token.type}`
            : message,
      });
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

      if (
        callback === undefined ||
        callback(input[get0(index)], index, input)
      ) {
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

  finishLoc(start: Position): SourceLocation {
    return this.finishLocAt(start, this.getEndPosition());
  }

  finishLocAt(start: Position, end: Position): SourceLocation {
    return {
      filename: this.filename,
      start,
      end,
    };
  }

  finalize() {
    if (!this.eatToken('EOF')) {
      throw this.unexpected({
        message: 'Expected end of file',
      });
    }
  }
}

export class ParserWithRequiredPath<
  Tokens extends TokensShape,
  State
> extends ParserCore<Tokens, State> {
  constructor(
    opts: ParserOptionsWithRequiredPath,
    parserName: string,
    initialState: State,
  ) {
    super(opts, parserName, initialState);
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

export function isESIdentifier(char: undefined | string): boolean {
  return char !== undefined && /[A-F0-9a-z_$]/.test(char);
}

export function isESIdentifierStart(char: undefined | string): boolean {
  return char !== undefined && /[A-Fa-z_$]/.test(char);
}

export function isEscaped(index: Number0, input: string) {
  const prevChar = input[get0(index) - 1];
  const prevPrevChar = input[get0(index) - 2];
  const isEscaped = prevChar === '\\' && prevPrevChar !== '\\';
  return isEscaped;
}

export function readUntilLineBreak(char: string): boolean {
  return char !== '\n';
}

// Lazy initialize a ParserCore subclass... Circular dependencies are wild and necessitate this as ParserCore may not be available
export function createParser<T, Args extends Array<unknown>>(
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
