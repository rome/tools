/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  assertNodeTypeSet,
  WithStatement,
  ForInStatement,
  ForOfStatement,
  ForStatement,
  WhileStatement,
} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';
import {AnyNode, AnyComment} from '@romejs/js-ast';
import generatorFunctions from './generators/index';
import * as n from './node/index';
import Buffer, {BufferSnapshot} from './Buffer';
import {Number0, get0, Number1} from '@romejs/ob1';
import {isTypeNode, isTypeExpressionWrapperNode} from '@romejs/js-ast-utils';
import {SourceMap} from '@romejs/codec-source-map';

assertNodeTypeSet(generatorFunctions, 'generators');

export type GeneratorMethod = (generator: Generator, node: AnyNode, parent: AnyNode) =>
  | void
  | never;

const SCIENTIFIC_NOTATION = /e/i;
const ZERO_DECIMAL_INTEGER = /\.0+$/;
const NON_DECIMAL_LITERAL = /^0[box]/;

export type GeneratorOptions = {
  typeAnnotations: boolean;
  format?: 'pretty' | 'compact';
  indent?: number;
  inputSourceMap?: SourceMap;
  sourceMapTarget?: string;
  sourceRoot?: string;
  sourceFileName?: string;
};

type TerminatorState = {printed: boolean};

type PrintJoinOptions<N> = {
  indent?: boolean;
  multiline?: boolean;
  noSeparatorLast?: boolean;
  after?: (generator: Generator, isLast: boolean) => void;
};

const MAX_PRETTY_LINE_LENGTH = 80;

function doesLineExceed(column: Number0): boolean {
  // We take off 1 as column is the column that the line ends, which is not a character
  return get0(column) - 1 > MAX_PRETTY_LINE_LENGTH;
}

type GeneratorSnapshot = {
  inForStatementInitCounter: number;
  printedCommentStarts: Set<Number0>;
  printedComments: Set<AnyComment>;
  inferredNewlines: Set<Number1>;
  printStackIndex: number;
  parenPushNewlineState: undefined | TerminatorState;
  endsWithInteger: boolean;
  endsWithWord: boolean;
  currentIndentLevel: number;
  currentLineIndentLevel: number;
  buffer: BufferSnapshot;
};

type MultilineCondition =
  | 'more-than-one-line'
  | 'any-line-exceeds'
  | 'source-had-multiline';

export default class Generator {
  constructor(opts: GeneratorOptions, code: string) {
    this.buf = new Buffer(opts, code);
    this.currentIndentLevel = opts.indent === undefined ? 0 : opts.indent;
    this.currentLineIndentLevel = this.currentIndentLevel;

    this.options = opts;
    this.inForStatementInitCounter = 0;
    this.inferredNewlines = new Set();
    this.printedCommentStarts = new Set();
    this.printedComments = new Set();
    this.printStack = [];
    this.parenPushNewlineState = undefined;
    this.endsWithInteger = false;
    this.endsWithWord = false;
  }

  options: GeneratorOptions;
  inForStatementInitCounter: number;
  printedCommentStarts: Set<Number0>;
  printedComments: Set<AnyComment>;
  buf: Buffer;
  printStack: Array<AnyNode>;
  parenPushNewlineState: undefined | TerminatorState;
  endsWithInteger: boolean;
  endsWithWord: boolean;
  currentIndentLevel: number;
  currentLineIndentLevel: number;
  inferredNewlines: Set<Number1>;

  indent(): void {
    this.currentIndentLevel++;
  }

  dedent(): void {
    this.currentIndentLevel--;
  }

  save(): GeneratorSnapshot {
    return {
      inForStatementInitCounter: this.inForStatementInitCounter,
      printedCommentStarts: new Set(this.printedCommentStarts),
      printedComments: new Set(this.printedComments),
      inferredNewlines: new Set(this.inferredNewlines),
      printStackIndex: this.printStack.length,
      parenPushNewlineState: this.parenPushNewlineState,
      endsWithInteger: this.endsWithInteger,
      endsWithWord: this.endsWithWord,
      currentIndentLevel: this.currentIndentLevel,
      currentLineIndentLevel: this.currentLineIndentLevel,
      buffer: this.buf.save(),
    };
  }

  restore(snapshot: GeneratorSnapshot) {
    this.buf.restore(snapshot.buffer);
    this.inForStatementInitCounter = snapshot.inForStatementInitCounter;
    this.printedCommentStarts = snapshot.printedCommentStarts;
    this.printedComments = snapshot.printedComments;
    this.inferredNewlines = snapshot.inferredNewlines;
    this.printStack = this.printStack.slice(0, snapshot.printStackIndex);
    this.parenPushNewlineState = snapshot.parenPushNewlineState;
    this.endsWithInteger = snapshot.endsWithInteger;
    this.endsWithWord = snapshot.endsWithWord;
    this.currentIndentLevel = snapshot.currentIndentLevel;
    this.currentLineIndentLevel = snapshot.currentLineIndentLevel;
  }

  multiline<N extends AnyNode>(
    node: N,
    callback: (multiline: boolean, node: N) => void,
    {
      conditions,
      indentTrailingNewline,
      indent,
    }: {
      indent?: boolean;
      indentTrailingNewline?: boolean;
      conditions?: Array<MultilineCondition>;
    } = {},
  ) {
    if (this.options.format !== 'pretty') {
      callback(n.isMultiLine(node), node);
      return;
    }

    // If we have the source-had-multiline condition and the original source had multiple lines then assume multiline always
    if (
      conditions !== undefined && conditions.includes('source-had-multiline') &&
        n.isMultiLine(node)
    ) {
      callback(true, node);
      return;
    }

    const snapshot = this.save();

    callback(false, node);

    let shouldMultiline = false;

    // AKA first-line-exceeds
    if (this.buf.position.line === snapshot.buffer.position.line) {
      shouldMultiline = doesLineExceed(this.buf.position.column);
    } else {
      shouldMultiline = doesLineExceed(
        this.buf.lineLengths[snapshot.buffer.lineLengthsIndex],
      );
    }

    if (conditions !== undefined && !shouldMultiline) {
      for (const condition of conditions) {
        switch (condition) {
          case 'more-than-one-line':
            shouldMultiline = this.buf.position.line !==
            snapshot.buffer.position.line;
            break;

          case 'any-line-exceeds':
            // Check current line
            shouldMultiline = doesLineExceed(this.buf.position.column);

            // Check previous lines
            for (
              let i = snapshot.buffer.lineLengthsIndex;
              i < this.buf.lineLengths.length;
              i++
            ) {
              if (shouldMultiline) {
                break;
              }
              shouldMultiline = doesLineExceed(this.buf.lineLengths[i]);
            }
            break;
        }

        if (shouldMultiline) {
          break;
        }
      }
    }

    if (shouldMultiline) {
      this.restore(snapshot);

      if (indent) {
        this.newline();
        this.indent();
      }

      callback(true, node);

      if (indent) {
        if (indentTrailingNewline) {
          this.newline();
        }
        this.dedent();
      }
    }
  }

  /**
   * Add a semicolon to the buffer.
   */
  semicolon(): void {
    this.append(';');
  }

  /**
   * Add a right brace to the buffer.
   */
  rightBrace(): void {
    // TODO remove this?
    this.token('}');
  }

  spaceOrNewline(newline: boolean) {
    if (newline) {
      this.newline();
    } else {
      this.space();
    }
  }

  /**
   * Add a space to the buffer unless it is compact.
   */
  space(force: boolean = false): void {
    if (this.buf.hasContent() && !this.buf.endsWith(' ') && !this.buf.endsWith(
      '\n',
    ) || force) {
      this._space();
    }
  }

  /**
   * Writes a token that can't be safely parsed without taking whitespace into account.
   */
  word(str: string): void {
    if (this.endsWithWord) {
      this.space();
    }

    this.append(str);

    this.endsWithWord = true;
  }

  /**
   * Writes a number token so that we can validate if it is an integer.
   */
  number(str: string): void {
    this.word(str);

    // Integer tokens need special handling because they cannot have '.'s inserted

    // immediately after them.
    this.endsWithInteger = Number.isInteger(Number(str)) &&
      !NON_DECIMAL_LITERAL.test(str) && !SCIENTIFIC_NOTATION.test(str) &&
      !ZERO_DECIMAL_INTEGER.test(str) && str[str.length - 1] !== '.';
  }

  /**
   * Writes a simple token.
   */
  token(str: string): void {
    // space is mandatory to avoid outputting <!--

    // http://javascript.spec.whatwg.org/#comment-syntax
    if (str === '--' && this.buf.endsWith('!') ||
    // Need spaces for operators of the same kind to avoid: `a+++b`
    str[0] === '+' && this.buf.endsWith('+') || str[0] === '-' &&
      this.buf.endsWith('-') ||
    // Needs spaces to avoid changing '34' to '34.', which would still be a valid number.
    str[0] === '.' && this.endsWithInteger) {
      this.space();
    }

    this.append(str);
  }

  source(prop: string, loc: undefined | SourceLocation): void {
    this.buf.source(prop, loc);
  }

  _space(): void {
    this.append(' ');
  }

  newline() {
    if (!this.buf.endsWith('\n')) {
      this.forceNewline();
    }
  }

  inferredNewline(line: Number1): boolean {
    if (this.inferredNewlines.has(line)) {
      return false;
    } else {
      this.inferredNewlines.add(line);
      this.forceNewline();
      return true;
    }
  }

  forceNewline(): void {
    if (this.buf.isEmpty()) {
      return;
    }

    // Never allow more than two lines
    if (this.buf.endsWith('\n\n')) {
      return;
    }

    //
    if (this.buf.endsWith('{\n') || this.buf.endsWith(':\n')) {
      return;
    }

    if (!this.parenPushNewlineState) {
      this.buf.removeTrailing(' ');
    }
    this.append('\n');
  }

  newlineX(num: number) {
    if (num >= 1) {
      this.forceNewline();
    }
    if (num >= 2) {
      this.forceNewline();
    }
  }

  append(str: string) {
    this.maybeAddParen(str);
    this.maybeIndent(str);
    this.buf.append(str);
    this.endsWithWord = false;
    this.endsWithInteger = false;
  }

  maybeIndent(str: string): void {
    // we've got a newline before us so prepend on the indentation
    if (this.buf.endsWith('\n') && str[0] !== '\n') {
      if (this.currentIndentLevel > 0) {
        this.buf.append(this.getIndent());
        this.currentLineIndentLevel = this.currentIndentLevel;
      } else {
        this.currentLineIndentLevel = 0;
      }
    }
  }

  maybeAddParen(str: string): void {
    // see startTerminatorless() instance method
    const parenPushNewlineState = this.parenPushNewlineState;
    if (!parenPushNewlineState) {
      return undefined;
    }
    this.parenPushNewlineState = undefined;

    let i;
    for (i = 0; i < str.length && str[i] === ' '; i++) {
      continue;
    }
    if (i === str.length) {
      return undefined;
    }

    const cha = str[i];
    if (cha === '\n' || cha === '/') {
      // we're going to break this terminator expression so we need to add a parentheses
      this.token('(');
      this.indent();
      parenPushNewlineState.printed = true;
    }
  }

  /**
   * Get the current indent.
   */
  getIndent(): string {
    return '  '.repeat(this.currentIndentLevel);
  }

  /**
   * Set some state that will be modified if a newline has been inserted before any
   * non-space characters.
   *
   * This is to prevent breaking semantics for terminatorless separator nodes. eg:
   *
   *    return foo;
   *
   * returns `foo`. But if we do:
   *
   *   return
   *   foo;
   *
   *  `undefined` will be returned and not `foo` due to the terminator.
   */
  startTerminatorless(): TerminatorState {
    return this.parenPushNewlineState = {
      printed: false,
    };
  }

  /**
   * Print an ending parentheses if a starting one has been printed.
   */
  endTerminatorless(state: TerminatorState) {
    if (state.printed) {
      this.dedent();
      this.forceNewline();
      this.token(')');
    }
  }

  // If the passed in node exists then print a colon followed by the node
  printTypeColon(node: undefined | AnyNode, parent: AnyNode) {
    if (node !== undefined) {
      this.token(':');
      this.space();
      this.print(node, parent);
    }
  }

  maybeCommentNewlines(
    node: AnyNode,
    comment: undefined | AnyComment,
    trailing: boolean,
  ) {
    if (comment === undefined) {
      return;
    }

    const lines = n.getLinesBetween(node, comment);

    // BlockComment already has a newline
    if (lines.length >= 1 && (comment.type !== 'CommentLine' || trailing)) {
      this.inferredNewline(lines[0]);
    }

    if (lines.length >= 2) {
      this.inferredNewline(lines[1]);
    }
  }

  print(
    node: undefined | AnyNode,
    parent: AnyNode,
    beforeTrailing?: () => void,
    includeTrailingComments: boolean = true,
  ) {
    if (node === undefined) {
      return;
    }

    if (this.options.typeAnnotations === false && isTypeNode(node) &&
      !isTypeExpressionWrapperNode(node)) {
      return;
    }

    const printMethod: undefined | GeneratorMethod = generatorFunctions.get(
      node.type,
    );
    if (printMethod === undefined) {
      throw new Error(
        `No known generator for node ${node.type} with parent ${parent.type}`,
      );
    }

    this.printStack.push(node);

    const needsParens = n.needsParens(node, parent, this.printStack);

    if (needsParens) {
      this.token('(');
    }

    const loc = node.type === 'Program' ? undefined : node.loc;

    const leadingComments = this.getComments(true, node);
    this.printComments(leadingComments);

    // If leading comment had an empty line after then retain it
    if (leadingComments !== undefined) {
      this.maybeCommentNewlines(
        node,
        leadingComments[leadingComments.length - 1],
        false,
      );
    }

    this.buf.withSource('start', loc, () => {
      printMethod(this, node, parent);
    });

    if (needsParens) {
      this.token(')');
    }

    if (beforeTrailing !== undefined) {
      beforeTrailing();
    }

    // If there's an empty line between the node and it's trailing comments then keep it
    if (includeTrailingComments) {
      const trailingComments = this.getComments(false, node);
      if (trailingComments !== undefined) {
        this.maybeCommentNewlines(node, trailingComments[0], true);
      }
      this.printComments(trailingComments);
    }

    this.printStack.pop();
  }

  printJoin<N extends AnyNode>(
    nodes: undefined | Array<undefined | N>,
    parent: AnyNode,
    opts: PrintJoinOptions<N> = {},
  ) {
    if (!nodes || !nodes.length) {
      return undefined;
    }

    if (opts.indent === true) {
      this.indent();
    }

    if (opts.multiline === true) {
      this.forceNewline();
    }

    let isLastNode = false;
    let i = 0;

    let printAfter;
    if (opts.after !== undefined) {
      printAfter = () => {
        if (opts.after) {
          opts.after(this, isLastNode);
        }
      };
    }

    while (i < nodes.length) {
      const node = nodes[i];
      isLastNode = i === nodes.length - 1;

      if (node === undefined) {
        if (printAfter !== undefined) {
          printAfter();
        }
      } else {
        this.print(node, parent, printAfter);

        if (opts.multiline === true) {
          let nextNode = (nodes[i + 1] as AnyNode);

          // Don't print a newline if the next node has a leadingComment that begins on the same line as this node
          let hasNextTrailingCommentOnSameLine = false;

          // Lots of refinements...
          if (nextNode !== undefined && node.loc !== undefined &&
            nextNode.loc !== undefined && nextNode.leadingComments !== undefined) {
            const firstNextNodeLeadingComments = nextNode.leadingComments[0];
            if (firstNextNodeLeadingComments !== undefined &&
              firstNextNodeLeadingComments.loc !== undefined) {
              nextNode = firstNextNodeLeadingComments;
              hasNextTrailingCommentOnSameLine = node.loc.end.line ===
              firstNextNodeLeadingComments.loc.start.line;
            }
          }

          if (!hasNextTrailingCommentOnSameLine) {
            this.newline();
          }

          this.maybeInsertExtraStatementNewlines(node, nextNode);
        }
      }

      i++;
    }

    if (opts.indent === true) {
      this.dedent();
    }
  }

  maybeInsertExtraStatementNewlines(node: AnyNode, nextNode: undefined | AnyNode) {
    // Insert an inferred newline or extra if it satisfies our conditions
    const linesBetween = n.getLinesBetween(node, nextNode);
    if (linesBetween.length > 1) {
      this.inferredNewline(linesBetween[1]);
      return;
    }

    if (n.hasExtraLineBetween(node)) {
      this.forceNewline();
    }
  }

  printBlock(
    parent:
      | WithStatement
      | ForInStatement
      | ForOfStatement
      | ForStatement
      | WhileStatement,
  ) {
    const node = parent.body;

    if (node.type !== 'EmptyStatement') {
      this.space();
    }

    this.print(node, parent);
  }

  printStatementList<N extends AnyNode>(
    nodes: undefined | Array<N>,
    parent: AnyNode,
    opts: PrintJoinOptions<N> = {},
  ) {
    return this.printJoin<N>(nodes, parent, {...opts, multiline: true});
  }

  printCommaList<N extends AnyNode>(
    items: undefined | Array<undefined | N>,
    parent: AnyNode,
    opts: {
      trailing?: boolean;
      multiline?: boolean;
    } = {},
  ) {
    if (!items || !items.length) {
      return undefined;
    }

    const print = (multiline: boolean) => {
      function separator(generator: Generator, isLast: boolean) {
        if (isLast && (!opts.trailing || !multiline)) {
          return;
        }

        generator.token(',');
        if (!multiline) {
          generator.space();
        }
      }

      this.printJoin<N>(items, parent, {
        after: separator,
        indent: multiline,
        multiline,
      });
    };

    if (opts.multiline === undefined) {
      this.multiline(parent, print);
    } else {
      print(opts.multiline);
    }
  }

  printInnerComments(node: AnyNode, indent: boolean = true) {
    const {innerComments} = node;
    if (innerComments === undefined) {
      return;
    }

    if (indent) {
      this.indent();
    }

    if (n.getLinesBetween(node, innerComments[0])) {
      this.forceNewline();
    }

    this.printComments(innerComments);

    if (indent) {
      this.dedent();
    }
  }

  printComments(comments: undefined | Array<AnyComment>) {
    if (!comments || !comments.length) {
      return undefined;
    }

    for (let i = 0; i < comments.length; i++) {
      const comment = comments[i];
      this.printComment(comment);

      const nextComment = comments[i + 1];
      if (nextComment !== undefined) {
        this.maybeCommentNewlines(comment, nextComment, true);
      }
    }
  }

  getComments(leading: boolean, node: AnyNode): undefined | Array<AnyComment> {
    if (!node) {
      return undefined;
    }

    const comments = leading ? node.leadingComments : node.trailingComments;
    if (!comments) {
      return undefined;
    }

    return comments.filter((comment: AnyComment) => {
      return !this.hasPrintedComment(comment);
    });
  }

  hasPrintedComment(comment: undefined | AnyComment): boolean {
    if (!comment) {
      return true;
    }

    if (this.printedComments.has(comment)) {
      return true;
    }

    if (comment.loc !== undefined && this.printedCommentStarts.has(
      comment.loc.start.index,
    )) {
      return true;
    }

    return false;
  }

  printComment(comment: AnyComment) {
    if (this.hasPrintedComment(comment)) {
      return;
    }

    this.printedComments.add(comment);

    if (comment.loc !== undefined) {
      this.printedCommentStarts.add(comment.loc.start.index);
    }

    if (!this.buf.endsWith('[') && !this.buf.endsWith('{')) {
      this.space();
    }

    // Avoid creating //* comments
    if (this.buf.endsWith('/')) {
      this.space();
    }

    this.buf.withSource('start', comment.loc, () => {
      const isBlockComment = comment.type === 'CommentBlock';
      const val = isBlockComment
        ? `/*${comment.value}*/` : `//${comment.value}\n`;
      this.append(val);
    });
  }
}
