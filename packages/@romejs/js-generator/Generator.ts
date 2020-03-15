/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  assertNodeTypeSet,
  MOCK_PARENT,
  WithStatement,
  ForInStatement,
  ForOfStatement,
  ForStatement,
  WhileStatement,
  AnyNode,
  AnyComment,
} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';
import generatorFunctions from './generators/index';
import * as n from './node/index';
import Buffer from './Buffer';
import {Number0} from '@romejs/ob1';
import {isTypeNode, isTypeExpressionWrapperNode} from '@romejs/js-ast-utils';
import {SourceMap} from '@romejs/codec-source-map';

assertNodeTypeSet(generatorFunctions, 'generators');

export type GeneratorMethod = (
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) => void | never;

const SCIENTIFIC_NOTATION = /e/i;
const ZERO_DECIMAL_INTEGER = /\.0+$/;
const NON_DECIMAL_LITERAL = /^0[box]/;

export type GeneratorOptions = {
  typeAnnotations: boolean;
  indent?: number;
  inputSourceMap?: SourceMap;
  sourceMapTarget?: string;
  sourceRoot?: string;
  sourceFileName?: string;
};

type TerminatorState = {
  printed: boolean;
};

type PrintJoinOptions<N> = {
  indent?: boolean;
  statement?: boolean;
  iterator?: (node: N, i: number) => void;
  separator?: (generator: Generator) => void;
};

type AnyNodeButComment = Exclude<AnyNode, AnyComment>;

export default class Generator {
  constructor(opts: GeneratorOptions, code: string) {
    this.buf = new Buffer(opts, code);
    this.currentIndentLevel = opts.indent === undefined ? 0 : opts.indent;

    this.options = opts;
    this.inForStatementInitCounter = 0;
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

  indent(): void {
    this.currentIndentLevel++;
  }

  dedent(): void {
    this.currentIndentLevel--;
  }

  /**
   * Add a semicolon to the buffer.
   */

  semicolon(force: boolean = false): void {
    this.append(';', !force /* queue */);
  }

  /**
   * Add a right brace to the buffer.
   */

  rightBrace(): void {
    this.token('}');
  }

  /**
   * Add a space to the buffer unless it is compact.
   */

  space(force: boolean = false): void {
    if (
      (this.buf.hasContent() && !this.endsWith(' ') && !this.endsWith('\n')) ||
      force
    ) {
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
    this.endsWithInteger =
      Number.isInteger(Number(str)) &&
      !NON_DECIMAL_LITERAL.test(str) &&
      !SCIENTIFIC_NOTATION.test(str) &&
      !ZERO_DECIMAL_INTEGER.test(str) &&
      str[str.length - 1] !== '.';
  }

  /**
   * Writes a simple token.
   */

  token(str: string): void {
    // space is mandatory to avoid outputting <!--
    // http://javascript.spec.whatwg.org/#comment-syntax
    if (
      (str === '--' && this.endsWith('!')) ||
      // Need spaces for operators of the same kind to avoid: `a+++b`
      (str[0] === '+' && this.endsWith('+')) ||
      (str[0] === '-' && this.endsWith('-')) ||
      // Needs spaces to avoid changing '34' to '34.', which would still be a valid number.
      (str[0] === '.' && this.endsWithInteger)
    ) {
      this.space();
    }

    this.append(str);
  }

  endsWith(str: string): boolean {
    return this.buf.endsWith(str);
  }

  removeTrailingNewline(): void {
    this.buf.removeTrailingNewline();
  }

  source(prop: string, loc: undefined | SourceLocation): void {
    this.buf.source(prop, loc);
  }

  _space(): void {
    this.append(' ', true /* queue */);
  }

  newline(): void {
    if (this.buf.isEmpty()) {
      return;
    }

    // Never allow more than two lines
    if (this.endsWith('\n\n')) {
      return;
    }

    //
    if (this.endsWith('{\n') || this.endsWith(':\n')) {
      return;
    }

    this.append('\n', true /* queue */);
  }

  newlineX(num: number) {
    for (let i = 0; i < num && i < 2; i++) {
      this.newline();
    }
  }

  append(str: string, queue: boolean = false) {
    this.maybeAddParen(str);
    this.maybeIndent(str);

    if (queue) {
      this.buf.queue(str);
    } else {
      this.buf.append(str);
    }

    this.endsWithWord = false;
    this.endsWithInteger = false;
  }

  maybeIndent(str: string): void {
    // we've got a newline before us so prepend on the indentation
    if (this.currentIndentLevel > 0 && this.endsWith('\n') && str[0] !== '\n') {
      this.buf.queue(this.getIndent());
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
    return (this.parenPushNewlineState = {
      printed: false,
    });
  }

  /**
   * Print an ending parentheses if a starting one has been printed.
   */

  endTerminatorless(state: TerminatorState) {
    if (state.printed) {
      this.dedent();
      this.newline();
      this.token(')');
    }
  }

  // If the passed in node exists then print a colon followed by the node
  printTypeColon(node: undefined | AnyNode, parent?: AnyNode) {
    if (node !== undefined) {
      this.token(':');
      this.space();
      this.print(node, parent);
    }
  }

  print(node: undefined | AnyNode, parent: AnyNode = MOCK_PARENT) {
    if (node === undefined) {
      return;
    }

    if (
      this.options.typeAnnotations === false &&
      isTypeNode(node) &&
      !isTypeExpressionWrapperNode(node)
    ) {
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

    if (needsParens === true) {
      this.token('(');
    }

    const loc = node.type === 'Program' ? undefined : node.loc;

    const leadingComments = this.getComments(true, node);
    this.printComments(leadingComments);

    // if leading comment had an empty line after then retain it
    const lastComment =
      leadingComments && leadingComments[leadingComments.length - 1];
    this.newlineX(n.getLinesBetween(lastComment, node) - 1);

    this.buf.withSource('start', loc, () => {
      printMethod(this, node, parent);
    });

    // if there's an empty line between the node and it's trailing comments then keep it
    const trailingComments = this.getComments(false, node);
    this.newlineX(
      n.getLinesBetween(trailingComments && trailingComments[0], node),
    );
    this.printComments(trailingComments);

    if (needsParens === true) {
      this.token(')');
    }

    // end
    this.printStack.pop();
  }

  getStatementList<N extends AnyNodeButComment>(
    nodes: Array<N>,
  ): Array<N | AnyComment> {
    const allNodes: Set<N | AnyComment> = new Set();

    for (const node of nodes) {
      this.getStatementList_addComments(allNodes, this.getComments(true, node));

      allNodes.add(node);

      this.getStatementList_addComments(
        allNodes,
        this.getComments(false, node),
      );
    }

    return Array.from(allNodes);
  }

  getStatementList_addComments<N>(
    allNodes: Set<N | AnyComment>,
    comments: undefined | Array<AnyComment>,
  ) {
    if (!comments) {
      return;
    }

    for (const comment of comments) {
      this.printedComments.add(comment);
      allNodes.add(comment);
    }
  }

  printJoin<N extends AnyNodeButComment>(
    nodes: undefined | Array<N>,
    parent: AnyNode,
    opts: PrintJoinOptions<N> = {},
  ) {
    if (!nodes || !nodes.length) {
      return undefined;
    }

    if (opts.indent === true) {
      this.indent();
    }

    if (opts.statement === true) {
      this.newline();
    }

    let interleavedNodes: Array<N | AnyComment> = nodes;
    if (opts.statement === true) {
      interleavedNodes = this.getStatementList(nodes);
    }

    for (let i = 0; i < interleavedNodes.length; i++) {
      const node = interleavedNodes[i];
      if (!node) {
        continue;
      }

      if (node.type === 'CommentBlock' || node.type === 'CommentLine') {
        this.printComment(node);
      } else {
        this.print(node, parent);

        if (opts.iterator) {
          opts.iterator(node, i);
        }

        const isLastNode = i === interleavedNodes.length - 1;
        if (opts.separator && !isLastNode) {
          opts.separator(this);
        }
      }

      if (opts.statement === true) {
        const nextNode = interleavedNodes[i + 1];
        if (nextNode) {
          if (node.loc && nextNode.loc) {
            let linesBetween = n.getLinesBetween(node, nextNode);
            if (node.type === 'CommentLine') {
              linesBetween--;
            }
            this.newlineX(linesBetween);
          } else {
            this.newline();
          }
        }
      }
    }

    if (opts.indent === true) {
      this.dedent();
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

  printStatementList<N extends AnyNodeButComment>(
    nodes: undefined | Array<N>,
    parent: AnyNode,
    opts: PrintJoinOptions<N> = {},
  ) {
    return this.printJoin<N>(nodes, parent, {...opts, statement: true});
  }

  printCommaList<N extends AnyNodeButComment>(
    items: undefined | Array<N>,
    parent: AnyNode,
    opts: PrintJoinOptions<N> = {},
  ) {
    if (!items || !items.length) {
      return undefined;
    }

    this.printJoin<N>(items, parent, {
      ...opts,
      separator: opts.separator || commaSeparator,
    });
  }

  printInnerComments(node: AnyNode, indent: boolean = true) {
    if (!node.innerComments) {
      return undefined;
    }

    if (indent) {
      this.indent();
    }

    this.printComments(node.innerComments);

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
      if (nextComment) {
        // comment lines always have a line after but comment blocks don't, only insert one if it's
        // present in the original source
        if (
          comment.type === 'CommentBlock' &&
          n.getLinesBetween(comment, nextComment) > 0
        ) {
          this.newline();
        }

        // extra newline between these two comments, retain it
        if (n.hasExtraLineBetween(comment, nextComment)) {
          this.newline();
        }
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

    if (
      comment.loc !== undefined &&
      this.printedCommentStarts.has(comment.loc.start.index)
    ) {
      return true;
    }

    return false;
  }

  printComment(comment: AnyComment) {
    this.printedComments.add(comment);

    if (comment.loc !== undefined) {
      this.printedCommentStarts.add(comment.loc.start.index);
    }

    const isBlockComment = comment.type === 'CommentBlock';
    const val = isBlockComment
      ? `/*${comment.value}*/`
      : `//${comment.value}\n`;

    if (!this.endsWith('[') && !this.endsWith('{')) {
      this.space();
    }

    // Avoid creating //* comments
    if (this.endsWith('/')) {
      this.space();
    }

    this.buf.withSource('start', comment.loc, () => {
      this.append(val);
    });
  }
}

function commaSeparator(generator: Generator) {
  generator.token(',');
  generator.space();
}
