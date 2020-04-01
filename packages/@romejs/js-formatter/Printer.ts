/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Tokens,
  Token,
  LinkedGroupsToken,
  GroupToken,
  newline,
  IndentToken,
  DerivedNewlineToken,
  NumberToken,
  TerminatorlessToken,
  CommentToken,
  WordToken,
  OperatorToken,
  VerbatimToken,
} from './tokens';
import {BuilderOptions} from './Builder';
import {Mappings} from '@romejs/codec-source-map';
import {Number1, Number0, number0, number1, get0, inc} from '@romejs/ob1';

type TerminatorState = {printed: boolean};

type State = {
  lastToken: Token;
  lastBuff: string;
  indentString: string;
  indentLevel: number;
  endsWithSpace: boolean;
  endsWithInteger: boolean;
  endsWithNewline: boolean;
  line: Number1;
  column: Number0;
  terminator?: TerminatorState;
};

type GroupSnapshot = {
  priority: boolean;
  state: State;
  buffIndex: number;
  tokens: Tokens;
  tokensIndex: number;
  lastUnbrokenGroup: undefined | GroupSnapshot;
  derivedNewlinesIndex: number;
  breakOnNewline: boolean;
};

class BreakGroupError extends Error {
  constructor(unbrokenGroup: GroupSnapshot) {
    super('Expected to be caught somewhere');
    this.unbrokenGroup = unbrokenGroup;
  }

  unbrokenGroup: GroupSnapshot;
}

const MAX_LINE_LENGTH = 80;

const SCIENTIFIC_NOTATION = /e/i;
const ZERO_DECIMAL_INTEGER = /\.0+$/;
const NON_DECIMAL_LITERAL = /^0[box]/;

export default class Printer {
  constructor(opts: BuilderOptions) {
    this.options = opts;

    this.state = {
      lastToken: newline,
      lastBuff: '',
      indentString: '',
      indentLevel: 0,
      endsWithSpace: false,
      endsWithInteger: false,
      endsWithNewline: true,
      column: number0,
      line: number1,
    };

    this.compact = this.options.format === 'compact';
    this.lineWrap = this.options.format === 'pretty';
    this.derivedNewlines = [];
    this.buff = [];
    this.mappings = [];
    this.lastUnbrokenGroup = undefined;
    this.brokenGroups = new Set();
  }

  compact: boolean;
  lineWrap: boolean;

  derivedNewlines: Array<number>;
  buff: Array<string>;
  mappings: Mappings;
  lastUnbrokenGroup: undefined | GroupSnapshot;
  brokenGroups: Set<LinkedGroupsToken | GroupToken>;

  options: BuilderOptions;
  state: State;

  maybeAddTerminatorlessParen(str: string) {
    const terminatorState = this.state.terminator;
    if (!terminatorState) {
      return;
    }

    this.state.terminator = undefined;

    let i;
    for (i = 0; i < str.length && str[i] === ' '; i++) {
      continue;
    }
    if (i === str.length) {
      return;
    }

    const cha = str[i];
    if (cha === '\n' || cha === '/') {
      // We're going to break this terminator expression so we need to add a parentheses
      this.push('(');
      this.indent();
      terminatorState.printed = true;
    }

    return;
  }

  push(str: string) {
    if (str === '') {
      return;
    }

    this.maybeAddTerminatorlessParen(str);

    // Only output indentation if we aren't compact
    if (!this.compact && str !== '\n' && this.state.endsWithNewline) {
      str = this.state.indentString + str;
    }

    // Determine if we need to line wrap. We skip this when we aren't in pretty mode for better performance.
    if (this.lineWrap) {
      const {lastUnbrokenGroup} = this;

      for (const char of str) {
        if (char === '\n') {
          if (
            lastUnbrokenGroup !== undefined &&
            lastUnbrokenGroup.breakOnNewline
          ) {
            throw new BreakGroupError(lastUnbrokenGroup);
          }
          this.state.column = number0;
          this.state.line = inc(this.state.line);
        } else {
          this.state.column = inc(this.state.column);
        }
      }

      if (
        lastUnbrokenGroup !== undefined &&
        get0(this.state.column) > MAX_LINE_LENGTH
      ) {
        throw new BreakGroupError(lastUnbrokenGroup);
      }
    }

    this.state.endsWithNewline = str[str.length - 1] === '\n';
    this.state.endsWithInteger = false;
    this.state.endsWithSpace = str[str.length - 1] === ' ';
    this.state.lastBuff = str;
    this.buff.push(str);
  }

  createStateSnapshot(
    priority: boolean,
    tokens: Tokens,
    index: number,
    breakOnNewline: boolean = false,
  ): GroupSnapshot {
    return {
      priority,
      tokens,
      tokensIndex: index,
      buffIndex: this.buff.length,
      state: {...this.state},
      lastUnbrokenGroup: this.lastUnbrokenGroup,
      derivedNewlinesIndex: this.derivedNewlines.length,
      breakOnNewline,
    };
  }

  restoreSnapshot(
    token: LinkedGroupsToken | GroupToken,
    snapshot: GroupSnapshot,
  ) {
    this.brokenGroups.add(token);
    this.lastUnbrokenGroup = snapshot.lastUnbrokenGroup;
    this.state = snapshot.state;
    this.buff = this.buff.slice(0, snapshot.buffIndex);
    this.derivedNewlines = this.derivedNewlines.slice(
      0,
      snapshot.derivedNewlinesIndex,
    );
    this.print(snapshot.tokens, snapshot.tokensIndex);
  }

  resetUnbrokenGroup(ourUnbrokenGroup: undefined | GroupSnapshot) {
    if (
      ourUnbrokenGroup !== undefined &&
      this.lastUnbrokenGroup === ourUnbrokenGroup
    ) {
      this.lastUnbrokenGroup = ourUnbrokenGroup.lastUnbrokenGroup;
    }
  }

  trim(str: string): boolean {
    const {buff} = this;
    if (buff[buff.length - 1] === str) {
      buff.pop();
      return true;
    } else {
      return false;
    }
  }

  canOverwriteLastUnbrokenGroup(): boolean {
    const {lastUnbrokenGroup} = this;

    if (lastUnbrokenGroup === undefined) {
      return true;
    }

    if (lastUnbrokenGroup.priority) {
      return false;
    }

    if (lastUnbrokenGroup.breakOnNewline) {
      return false;
    }

    return true;
  }

  newline() {
    while (
      // Remove all trailing spaces
      this.trim(' ')
    );
    this.push('\n');
  }

  updateIndentString() {
    this.state.indentString = '  '.repeat(this.state.indentLevel);
  }

  indent() {
    this.state.indentLevel++;
    this.updateIndentString();
  }

  dedent() {
    this.state.indentLevel--;
    this.updateIndentString();
  }

  printTerminatorlessToken(token: TerminatorlessToken) {
    const terminatorState: TerminatorState = {
      printed: false,
    };
    this.state.terminator = terminatorState;
    this.print(token.tokens);
    if (terminatorState.printed) {
      this.dedent();
      this.newline();
      this.push(')');
    }
  }

  printIndentToken(token: IndentToken) {
    this.indent();
    this.print(token.tokens);
    this.dedent();
  }

  printDerivedNewlineToken(token: DerivedNewlineToken) {
    if (!this.compact && !this.derivedNewlines.includes(token.id)) {
      this.newline();
      this.derivedNewlines.push(token.id);
    }
  }

  printNewlineToken() {
    if (!this.state.endsWithNewline && !this.compact) {
      this.newline();
    }
  }

  printSpaceToken() {
    if (!this.state.endsWithSpace) {
      this.push(' ');
    }
  }

  printCommentToken(token: CommentToken) {
    if (!this.compact) {
      // There might actually be comments we want to keep. TODO add some heuristics, licenses etc.
      if (!this.state.endsWithSpace && !this.state.endsWithNewline) {
        this.push(' ');
      }
      this.push(token.value);
    }
  }

  printNumberToken(token: NumberToken) {
    const str = token.value;
    this.push(str);

    this.state.endsWithInteger =
      Number.isInteger(Number(str)) &&
      !NON_DECIMAL_LITERAL.test(str) &&
      !SCIENTIFIC_NOTATION.test(str) &&
      !ZERO_DECIMAL_INTEGER.test(str) &&
      str[str.length - 1] !== '.';
  }

  printWordToken(token: WordToken) {
    if (this.state.lastToken.type === 'Word') {
      this.push(' ');
    }
    this.push(token.value);
  }

  printOperatorToken(token: OperatorToken) {
    const str = token.value;

    // Space is mandatory to avoid outputting <!--
    // http://javascript.spec.whatwg.org/#comment-syntax
    if (
      (str === '--' && this.state.lastBuff.endsWith('!')) || // Need spaces for operators of the same kind to avoid: `a+++b`
      (str[0] === '+' && this.state.lastBuff.endsWith('+')) ||
      (str[0] === '-' && this.state.lastBuff.endsWith('-')) || // Needs spaces to avoid changing '34' to '34.', which would still be a valid number.
      (str[0] === '.' && this.state.endsWithInteger)
    ) {
      this.push(' ');
    }

    this.push(str);
  }

  // A Group defines a boundary where we can break
  printGroupToken(token: GroupToken, tokens: Tokens, index: number) {
    const {breakOnNewline, groups, priority, broken, unbroken} = token;

    const isBroken = broken.force || this.brokenGroups.has(token);

    let ourUnbrokenGroup: undefined | GroupSnapshot;

    // If the last broken group was a linked group then it's in charge of us, so don't catch anything
    if (!isBroken && this.canOverwriteLastUnbrokenGroup()) {
      ourUnbrokenGroup = this.createStateSnapshot(
        priority === true,
        tokens,
        index,
        breakOnNewline,
      );
      this.lastUnbrokenGroup = ourUnbrokenGroup;
    }

    const shouldIndent = isBroken && broken.indent !== false;

    if (isBroken) {
      this.print(broken.before);
    }

    if (shouldIndent) {
      this.indent();
      if (broken.indentNewline !== false) {
        this.newline();
      }
    }

    try {
      for (let i = 0; i < groups.length; i++) {
        if (i === 0) {
          this.print(isBroken ? broken.leading : unbroken.leading);
        }

        let group = groups[i];
        if (Array.isArray(group)) {
          group = {tokens: group};
        }

        this.print(group.tokens);

        const isLastNode = i === groups.length - 1;
        if (isLastNode) {
          this.print(isBroken ? broken.trailing : unbroken.trailing);
        } else {
          this.print(isBroken ? broken.separator : unbroken.separator);
        }

        this.print(isBroken ? group.afterBroken : group.afterUnbroken);
      }
    } catch (err) {
      if (
        err instanceof BreakGroupError &&
        err.unbrokenGroup === ourUnbrokenGroup
      ) {
        this.restoreSnapshot(token, ourUnbrokenGroup);
        return;
      } else {
        throw err;
      }
    }

    this.resetUnbrokenGroup(ourUnbrokenGroup);

    if (shouldIndent) {
      if (broken.indentNewline !== false) {
        this.newline();
      }
      this.dedent();
    }

    if (!isBroken && unbroken.trim !== undefined) {
      this.trim(unbroken.trim);
    }

    if (isBroken) {
      this.print(broken.after);
    }
  }

  // Any group catchers inside a LinkedGroups will be deactivated. When the LinkedGroup is triggered it goes through the direct
  // group descendents and breaks each group in order.
  printLinkedGroupsToken(
    token: LinkedGroupsToken,
    tokens: Tokens,
    index: number,
  ) {
    if (this.lineWrap && this.canOverwriteLastUnbrokenGroup()) {
      let firstGroup: undefined | LinkedGroupsToken | GroupToken;
      // Get the first unbroken group
      for (const tok of token.tokens) {
        if (
          (tok.type === 'LinkedGroups' || tok.type === 'Group') &&
          !this.brokenGroups.has(tok) &&
          (tok.type !== 'Group' || !tok.broken.force)
        ) {
          firstGroup = tok;
          break;
        }
      }

      if (firstGroup !== undefined) {
        const snapshot = this.createStateSnapshot(true, tokens, index);
        this.lastUnbrokenGroup = snapshot;

        try {
          this.print(token.tokens);
        } catch (err) {
          if (
            err instanceof BreakGroupError &&
            err.unbrokenGroup === snapshot
          ) {
            this.brokenGroups.add(firstGroup);
            this.restoreSnapshot(token, snapshot);
            return;
          } else {
            throw err;
          }
        }

        this.resetUnbrokenGroup(snapshot);

        return;
      }
    }

    this.print(token.tokens);
  }

  printVerbatimToken(token: VerbatimToken) {
    this.push(token.value);
  }

  print(tokens: undefined | Tokens, i: number = 0) {
    if (tokens === undefined) {
      return;
    }

    for (; i < tokens.length; i++) {
      const token: Token = tokens[i];

      switch (token.type) {
        case 'Terminatorless':
          this.printTerminatorlessToken(token);
          break;

        case 'Indent':
          this.printIndentToken(token);
          break;

        case 'DerivedNewline':
          this.printDerivedNewlineToken(token);
          break;

        case 'Newline':
          this.printNewlineToken();
          break;

        case 'Space':
          this.printSpaceToken();
          break;

        case 'Comment':
          this.printCommentToken(token);
          break;

        case 'Number':
          this.printNumberToken(token);
          break;

        case 'Word':
          this.printWordToken(token);
          break;

        case 'Operator':
          this.printOperatorToken(token);
          break;

        case 'Group':
          this.printGroupToken(token, tokens, i);
          break;

        case 'LinkedGroups':
          this.printLinkedGroupsToken(token, tokens, i);
          break;

        case 'Verbatim':
          this.printVerbatimToken(token);
          break;
      }

      this.state.lastToken = token;
    }
  }

  getCode(): string {
    return this.buff.join('');
  }

  getMappings(): Mappings {
    return this.mappings.slice();
  }
}
