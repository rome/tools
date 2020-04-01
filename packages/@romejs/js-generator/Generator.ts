/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  assertNodeTypeSet,
  AnyNode,
  MOCK_PARENT,
  AnyComment,
} from '@romejs/js-ast';
import generatorFunctions from './generators/index';
import * as n from './node/index';
import {isTypeNode, isTypeExpressionWrapperNode} from '@romejs/js-ast-utils';
import {SourceMap, Mappings} from '@romejs/codec-source-map';
import {
  Tokens,
  Token,
  GroupToken,
  operator,
  space,
  newline,
  derivedNewline,
  indent,
  comment,
  LinkedGroupsToken,
} from './tokens';
import {Number0} from '@romejs/ob1';

assertNodeTypeSet(generatorFunctions, 'generators');

export type GeneratorMethod = (
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) => Tokens | never;

export type GeneratorOptions = {
  typeAnnotations: boolean;
  format?: 'pretty' | 'compact';
  indent?: number;
  inputSourceMap?: SourceMap;
  sourceMapTarget?: string;
  sourceRoot?: string;
  sourceFileName?: string;
};

type PrintJoinOptions = Omit<GroupToken, 'type' | 'groups'> & {
  newline: boolean;
};

type TerminatorState = {printed: boolean};

type State = {
  lastToken: Token;
  lastBuff: string;
  indentString: string;
  indentLevel: number;
  endsWithSpace: boolean;
  endsWithInteger: boolean;
  endsWithNewline: boolean;
  column: number;
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

function checkTrailingCommentsSameLine(
  node: AnyNode,
  nextNode: undefined | AnyNode,
): {
  hasNextTrailingCommentOnSameLine: boolean;
  nextNode: undefined | AnyNode;
} {
  // Don't print a newline if the next node has a leadingComment that begins on the same line as this node
  let hasNextTrailingCommentOnSameLine = false;

  // Lots of refinements...
  if (
    nextNode !== undefined &&
    node.loc !== undefined &&
    nextNode.loc !== undefined &&
    nextNode.leadingComments !== undefined
  ) {
    const firstNextNodeLeadingComments = nextNode.leadingComments[0];
    if (
      firstNextNodeLeadingComments !== undefined &&
      firstNextNodeLeadingComments.loc !== undefined
    ) {
      nextNode = firstNextNodeLeadingComments;
      hasNextTrailingCommentOnSameLine =
        node.loc.end.line === firstNextNodeLeadingComments.loc.start.line;
    }
  }

  return {nextNode, hasNextTrailingCommentOnSameLine};
}

const MAX_LINE_LENGTH = 80;

const SCIENTIFIC_NOTATION = /e/i;
const ZERO_DECIMAL_INTEGER = /\.0+$/;
const NON_DECIMAL_LITERAL = /^0[box]/;

export default class Generator {
  constructor(opts: GeneratorOptions, ast: AnyNode, code: string) {
    this.options = opts;
    this.inForStatementInitCounter = 0;
    this.printStack = [];
    this.printedCommentStarts = new Set();
    this.printedComments = new Set();
    this.tokens = this.print(ast, MOCK_PARENT);
  }

  tokens: Tokens;
  options: GeneratorOptions;
  printStack: Array<AnyNode>;
  inForStatementInitCounter: number;
  printedCommentStarts: Set<Number0>;
  printedComments: Set<AnyComment>;

  print(node: undefined | AnyNode, parent: AnyNode): Tokens {
    if (node === undefined) {
      return [];
    }

    if (
      this.options.typeAnnotations === false &&
      isTypeNode(node) &&
      !isTypeExpressionWrapperNode(node)
    ) {
      return [];
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

    let tokens: Tokens = [];

    const needsParens = n.needsParens(node, parent, this.printStack);

    if (needsParens) {
      tokens.push(operator('('));
    }

    const leadingComments = this.getComments(true, node);

    // If leading comment had an empty line after then retain it
    if (leadingComments !== undefined) {
      tokens = [
        ...tokens,
        ...this.printComments(leadingComments),
        ...this.maybeCommentNewlines(
          node,
          leadingComments[leadingComments.length - 1],
          false,
        ),
      ];
    }

    tokens = [...tokens, ...printMethod(this, node, parent)];

    if (needsParens) {
      tokens.push(operator(')'));
    }

    // If there's an empty line between the node and it's trailing comments then keep it
    const trailingComments = this.getComments(false, node);
    if (trailingComments !== undefined) {
      tokens = tokens.concat(
        this.maybeCommentNewlines(node, trailingComments[0], true),
      );
    }
    tokens = tokens.concat(this.printComments(trailingComments));

    this.printStack.pop();

    return tokens;
  }

  printJoin(
    nodes: undefined | Array<undefined | AnyNode>,
    parent: AnyNode,
    opts: PrintJoinOptions,
  ): GroupToken {
    const groups: GroupToken['groups'] = [];

    let forceBroken =
      opts.broken.force || (opts.breakOnNewline && n.isMultiLine(parent));

    if (nodes !== undefined) {
      for (let i = 0; i < nodes.length; i++) {
        const node = nodes[i];

        if (node === undefined) {
          groups.push([]);
        } else {
          const isLastNode = i === nodes.length - 1;

          const {
            nextNode,
            hasNextTrailingCommentOnSameLine,
          } = checkTrailingCommentsSameLine(node, nodes[i + 1]);

          const afterBroken: Tokens = [];
          if (
            !isLastNode &&
            opts.newline &&
            !hasNextTrailingCommentOnSameLine
          ) {
            afterBroken.push(newline);
          }

          const newlines = this.maybeInsertExtraStatementNewlines(
            node,
            nextNode,
          );

          groups.push({
            tokens: this.print(node, parent),
            afterBroken: [...afterBroken, ...newlines],
            afterUnbroken: newlines,
          });
        }
      }
    }

    return {
      type: 'Group',
      groups,
      ...opts,
      broken: {
        ...opts.broken,
        force: forceBroken,
      },
    };
  }

  printCommaList(
    items: undefined | Array<undefined | AnyNode>,
    parent: AnyNode,
    opts: {
      breakOnNewline?: boolean;
      trailing?: boolean;
      forceBroken?: boolean;
      indentNewline?: boolean;
      indent?: boolean;
    } = {},
  ): GroupToken {
    return this.printJoin(items, parent, {
      breakOnNewline: opts.breakOnNewline,
      newline: true,
      broken: {
        force: opts.forceBroken,
        indent: opts.indent,
        indentNewline: opts.indentNewline,
        separator: [operator(',')],
        trailing: opts.trailing === true ? [operator(',')] : [],
      },
      unbroken: {
        separator: [operator(','), space],
      },
    });
  }

  printStatementList(
    nodes: undefined | Array<undefined | AnyNode>,
    parent: AnyNode,
    shouldIndent: boolean = false,
  ): Tokens {
    let tokens: Tokens = [];
    if (nodes === undefined || nodes.length === 0) {
      return tokens;
    }

    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      if (node === undefined) {
        continue;
      }

      tokens = [...tokens, ...this.print(node, parent)];

      const {
        nextNode,
        hasNextTrailingCommentOnSameLine,
      } = checkTrailingCommentsSameLine(node, nodes[i + 1]);

      if (!hasNextTrailingCommentOnSameLine) {
        tokens.push(newline);
      }

      tokens = tokens.concat(
        this.maybeInsertExtraStatementNewlines(node, nextNode),
      );
    }

    if (shouldIndent) {
      tokens = [newline, indent(tokens), newline];
    }

    return tokens;
  }

  printTypeColon(node: undefined | AnyNode, parent: AnyNode): Tokens {
    if (node === undefined) {
      return [];
    } else {
      return [operator(':'), space, ...this.print(node, parent)];
    }
  }

  getCode(): string {
    const compact = this.options.format === 'compact';
    const lineWrap = this.options.format === 'pretty';

    let derivedNewlines: Array<number> = [];
    let buff: Array<string> = [];
    let lastUnbrokenGroup: undefined | GroupSnapshot;
    const brokenGroups: Set<LinkedGroupsToken | GroupToken> = new Set();

    let state: State = {
      lastToken: space,
      lastBuff: '',
      indentString: '',
      indentLevel: 0,
      endsWithSpace: false,
      endsWithInteger: false,
      endsWithNewline: true,
      column: 0,
    };

    function maybeAddTerminatorlessParen(str: string) {
      const terminatorState = state.terminator;
      if (!terminatorState) {
        return;
      }

      state.terminator = undefined;

      let i;
      for (i = 0; i < str.length && str[i] === ' '; i++) {
        continue;
      }
      if (i === str.length) {
        return undefined;
      }

      const cha = str[i];
      if (cha === '\n' || cha === '/') {
        // We're going to break this terminator expression so we need to add a parentheses
        push('(');
        indent();
        terminatorState.printed = true;
      }
    }

    function push(str: string) {
      if (str === '') {
        return;
      }

      maybeAddTerminatorlessParen(str);

      // Only output indentation if we aren't compact
      if (!compact && str !== '\n' && state.endsWithNewline) {
        str = state.indentString + str;
      }

      // Determine if we need to line wrap. We skip this when we aren't in pretty mode for better performance.
      if (lineWrap) {
        for (const char of str) {
          if (char === '\n') {
            if (
              lastUnbrokenGroup !== undefined &&
              lastUnbrokenGroup.breakOnNewline
            ) {
              throw new BreakGroupError(lastUnbrokenGroup);
            }
            state.column = 0;
          } else {
            state.column++;
          }
        }

        if (lastUnbrokenGroup !== undefined && state.column > MAX_LINE_LENGTH) {
          throw new BreakGroupError(lastUnbrokenGroup);
        }
      }

      state.endsWithNewline = str[str.length - 1] === '\n';
      state.endsWithInteger = false;
      state.endsWithSpace = str[str.length - 1] === ' ';
      state.lastBuff = str;
      buff.push(str);
    }

    function createStateSnapshot(
      priority: boolean,
      tokens: Tokens,
      i: number,
      breakOnNewline: boolean = false,
    ): GroupSnapshot {
      return {
        priority,
        tokens,
        tokensIndex: i,
        buffIndex: buff.length,
        state: {...state},
        lastUnbrokenGroup,
        derivedNewlinesIndex: derivedNewlines.length,
        breakOnNewline,
      };
    }

    function restoreSnapshot(
      token: LinkedGroupsToken | GroupToken,
      snapshot: GroupSnapshot,
    ) {
      brokenGroups.add(token);

      lastUnbrokenGroup = snapshot.lastUnbrokenGroup;
      state = snapshot.state;
      buff = buff.slice(0, snapshot.buffIndex);
      derivedNewlines = derivedNewlines.slice(0, snapshot.derivedNewlinesIndex);
      print(snapshot.tokens, snapshot.tokensIndex);
    }

    function resetUnbrokenGroup(ourUnbrokenGroup: undefined | GroupSnapshot) {
      if (
        ourUnbrokenGroup !== undefined &&
        lastUnbrokenGroup === ourUnbrokenGroup
      ) {
        lastUnbrokenGroup = ourUnbrokenGroup.lastUnbrokenGroup;
      }
    }

    function trim(str: string): boolean {
      if (buff[buff.length - 1] === str) {
        buff.pop();
        return true;
      } else {
        return false;
      }
    }

    function canOverwriteLastUnbrokenGroup(): boolean {
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

    function newline() {
      // Remove all trailing spaces
      while (trim(' '));
      push('\n');
    }

    function updateIndentString() {
      state.indentString = '  '.repeat(state.indentLevel);
    }

    function indent() {
      state.indentLevel++;
      updateIndentString();
    }

    function dedent() {
      state.indentLevel--;
      updateIndentString();
    }

    function print(tokens: undefined | Tokens, i: number = 0) {
      if (tokens === undefined) {
        return;
      }

      for (; i < tokens.length; i++) {
        const token: Token = tokens[i];

        switch (token.type) {
          case 'Terminatorless':
            const terminatorState: TerminatorState = {
              printed: false,
            };
            state.terminator = terminatorState;
            print(token.tokens);
            if (terminatorState.printed) {
              dedent();
              newline();
              push(')');
            }
            break;

          case 'Indent':
            indent();
            print(token.tokens);
            dedent();
            break;

          case 'DerivedNewline':
            if (!compact && !derivedNewlines.includes(token.id)) {
              newline();
              derivedNewlines.push(token.id);
            }
            break;

          case 'Newline':
            if (!state.endsWithNewline && !compact) {
              newline();
            }
            break;

          case 'Space':
            if (!state.endsWithSpace) {
              push(' ');
            }
            break;

          case 'Comment':
            if (!compact) {
              // There might actually be comments we want to keep. TODO add some heuristics, licenses etc.
              if (!state.endsWithSpace && !state.endsWithNewline) {
                push(' ');
              }
              push(token.value);
            }
            break;

          case 'Number': {
            const str = token.value;
            push(str);

            state.endsWithInteger =
              Number.isInteger(Number(str)) &&
              !NON_DECIMAL_LITERAL.test(str) &&
              !SCIENTIFIC_NOTATION.test(str) &&
              !ZERO_DECIMAL_INTEGER.test(str) &&
              str[str.length - 1] !== '.';
            break;
          }

          case 'Word':
            if (state.lastToken.type === 'Word') {
              push(' ');
            }
            push(token.value);
            break;

          case 'Operator': {
            const str = token.value;

            // Space is mandatory to avoid outputting <!--
            // http://javascript.spec.whatwg.org/#comment-syntax
            if (
              (str === '--' && state.lastBuff.endsWith('!')) ||
              // Need spaces for operators of the same kind to avoid: `a+++b`
              (str[0] === '+' && state.lastBuff.endsWith('+')) ||
              (str[0] === '-' && state.lastBuff.endsWith('-')) ||
              // Needs spaces to avoid changing '34' to '34.', which would still be a valid number.
              (str[0] === '.' && state.endsWithInteger)
            ) {
              push(' ');
            }

            push(str);
            break;
          }

          // A Group defines a boundary where we can break
          case 'Group': {
            const {breakOnNewline, groups, priority, broken, unbroken} = token;

            const isBroken = broken.force || brokenGroups.has(token);

            let ourUnbrokenGroup: undefined | GroupSnapshot;

            // If the last broken group was a linked group then it's in charge of us, so don't catch anything
            if (!isBroken && canOverwriteLastUnbrokenGroup()) {
              ourUnbrokenGroup = createStateSnapshot(
                priority === true,
                tokens,
                i,
                breakOnNewline,
              );
              lastUnbrokenGroup = ourUnbrokenGroup;
            }

            const shouldIndent = isBroken && broken.indent !== false;

            if (isBroken) {
              print(broken.before);
            }

            if (shouldIndent) {
              indent();
              if (broken.indentNewline !== false) {
                newline();
              }
            }

            try {
              for (let i = 0; i < groups.length; i++) {
                if (i === 0) {
                  print(isBroken ? broken.leading : unbroken.leading);
                }

                let group = groups[i];
                if (Array.isArray(group)) {
                  group = {tokens: group};
                }

                print(group.tokens);

                const isLastNode = i === groups.length - 1;
                if (isLastNode) {
                  print(isBroken ? broken.trailing : unbroken.trailing);
                } else {
                  print(isBroken ? broken.separator : unbroken.separator);
                }

                print(isBroken ? group.afterBroken : group.afterUnbroken);
              }
            } catch (err) {
              if (
                err instanceof BreakGroupError &&
                err.unbrokenGroup === ourUnbrokenGroup
              ) {
                restoreSnapshot(token, ourUnbrokenGroup);
                return;
              } else {
                throw err;
              }
            }

            resetUnbrokenGroup(ourUnbrokenGroup);

            if (shouldIndent) {
              if (broken.indentNewline !== false) {
                newline();
              }
              dedent();
            }

            if (!isBroken && unbroken.trim !== undefined) {
              trim(unbroken.trim);
            }

            if (isBroken) {
              print(broken.after);
            }

            break;
          }

          // Any group catchers inside a LinkedGroups will be deactivated. When the LinkedGroup is triggered it goes through the direct
          // group descendents and breaks each group in order.
          case 'LinkedGroups':
            if (lineWrap && canOverwriteLastUnbrokenGroup()) {
              let firstGroup: undefined | LinkedGroupsToken | GroupToken;
              // Get the first unbroken group
              for (const tok of token.tokens) {
                if (
                  (tok.type === 'LinkedGroups' || tok.type === 'Group') &&
                  !brokenGroups.has(tok) &&
                  (tok.type !== 'Group' || !tok.broken.force)
                ) {
                  firstGroup = tok;
                  break;
                }
              }

              if (firstGroup !== undefined) {
                const snapshot = createStateSnapshot(true, tokens, i);
                lastUnbrokenGroup = snapshot;

                try {
                  print(token.tokens);
                } catch (err) {
                  if (
                    err instanceof BreakGroupError &&
                    err.unbrokenGroup === snapshot
                  ) {
                    brokenGroups.add(firstGroup);
                    restoreSnapshot(token, snapshot);
                    return;
                  } else {
                    throw err;
                  }
                }

                resetUnbrokenGroup(snapshot);

                break;
              }
            }

            print(token.tokens);
            break;

          case 'Verbatim':
            push(token.value);
            break;
        }

        state.lastToken = token;
      }
    }

    print(this.tokens);

    return buff.join('');
  }

  getMappings(): Mappings {
    return [];
  }

  printInnerComments(node: AnyNode): Tokens {
    const {innerComments} = node;
    if (innerComments === undefined) {
      return [];
    }

    let tokens: Tokens = [];

    if (n.getLinesBetween(node, innerComments[0])) {
      tokens.push(newline);
    }

    return [...tokens, ...this.printComments(innerComments)];
  }

  printComments(comments: undefined | Array<AnyComment>): Tokens {
    if (!comments || !comments.length) {
      return [];
    }

    let tokens: Tokens = [];

    for (let i = 0; i < comments.length; i++) {
      const comment = comments[i];
      tokens = tokens.concat(this.printComment(comment));

      const nextComment = comments[i + 1];
      if (nextComment !== undefined) {
        tokens = tokens.concat(
          this.maybeCommentNewlines(comment, nextComment, true),
        );
      }
    }

    return tokens;
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

  printComment(node: AnyComment): Tokens {
    if (this.hasPrintedComment(node)) {
      return [];
    }

    this.printedComments.add(node);

    if (node.loc !== undefined) {
      this.printedCommentStarts.add(node.loc.start.index);
    }

    const isBlockComment = node.type === 'CommentBlock';
    const val = isBlockComment ? `/*${node.value}*/` : `//${node.value}\n`;
    return [comment(val)];
  }

  maybeCommentNewlines(
    node: AnyNode,
    comment: undefined | AnyComment,
    trailing: boolean,
  ): Tokens {
    if (comment === undefined) {
      return [];
    }

    const lines = n.getLinesBetween(node, comment);

    // Will always have at least one newline
    if (
      node.type === 'CommentLine' ||
      (comment.type === 'CommentLine' && !trailing)
    ) {
      lines.shift();
    }

    let tokens: Tokens = [];

    if (lines.length >= 1) {
      tokens.push(derivedNewline(lines[0]));
    }

    if (lines.length >= 2) {
      tokens.push(derivedNewline(lines[1]));
    }

    return tokens;
  }

  maybeInsertExtraStatementNewlines(
    node: AnyNode,
    nextNode: undefined | AnyNode,
  ): Tokens {
    // Insert an inferred newline or extra if it satisfies our conditions
    const linesBetween = n.getLinesBetween(node, nextNode);
    if (linesBetween.length > 1) {
      return [derivedNewline(linesBetween[1])];
    }

    if (n.hasExtraLineBetween(node)) {
      //this.forceNewline();
    }

    return [];
  }
}
