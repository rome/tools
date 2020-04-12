/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyComment} from '@romejs/js-ast';
import builderFunctions from './builders/index';
import * as n from './node/index';
import {isTypeNode, isTypeExpressionWrapperNode} from '@romejs/js-ast-utils';
import {SourceMap} from '@romejs/codec-source-map';
import {
  Tokens,
  GroupToken,
  operator,
  space,
  newline,
  derivedNewline,
  indent,
  comment,
  positionMarker,
  concat,
} from './tokens';
import {Number0} from '@romejs/ob1';

export type BuilderMethod = (
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
) => Tokens | never;

export type BuilderOptions = {
  typeAnnotations: boolean;
  format?: 'pretty' | 'compact';
  indent?: number;
  sourceMaps?: boolean;
  inputSourceMap?: SourceMap;
  sourceMapTarget?: string;
  sourceRoot?: string;
  sourceFileName?: string;
  sourceText?: string;
};

type PrintJoinOptions = Omit<GroupToken, 'type' | 'groups'> & {
  newline: boolean;
};

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
  if (nextNode !== undefined && node.loc !== undefined && nextNode.loc !==
      undefined && nextNode.leadingComments !== undefined) {
    const firstNextNodeLeadingComments = nextNode.leadingComments[0];
    if (firstNextNodeLeadingComments !== undefined &&
          firstNextNodeLeadingComments.loc !==
          undefined) {
      nextNode = firstNextNodeLeadingComments;
      hasNextTrailingCommentOnSameLine = node.loc.end.line ===
        firstNextNodeLeadingComments.loc.start.line;
    }
  }

  return {nextNode, hasNextTrailingCommentOnSameLine};
}

export default class Builder {
  constructor(opts: BuilderOptions) {
    this.options = opts;
    this.inForStatementInitCounter = 0;
    this.printStack = [];
    this.printedCommentStarts = new Set();
    this.printedComments = new Set();
  }

  options: BuilderOptions;
  printStack: Array<AnyNode>;
  inForStatementInitCounter: number;
  printedCommentStarts: Set<Number0>;
  printedComments: Set<AnyComment>;

  tokenize(node: undefined | AnyNode, parent: AnyNode): Tokens {
    if (node === undefined) {
      return [];
    }

    if (this.options.typeAnnotations === false && isTypeNode(node) &&
        !isTypeExpressionWrapperNode(node)) {
      return [];
    }

    const tokenizeNode: undefined | BuilderMethod = builderFunctions.get(
      node.type,
    );
    if (tokenizeNode === undefined) {
      throw new Error(
        `No known builder for node ${node.type} with parent ${parent.type}`,
      );
    }

    this.printStack.push(node);

    const tokens: Tokens = [];

    const needsParens = n.needsParens(node, parent, this.printStack);

    if (needsParens) {
      tokens.push(operator('('));
    }

    const leadingComments = this.getComments(true, node);

    // If leading comment had an empty line after then retain it
    if (leadingComments !== undefined) {
      tokens.push(concat(this.tokenizeComments(leadingComments)), concat(
        this.maybeCommentNewlines(node, leadingComments[leadingComments.length -
          1], false),
      ));
    }

    const nodeTokens = tokenizeNode(this, node, parent);
    if (node.loc === undefined || !this.options.sourceMaps) {
      tokens.push(concat(nodeTokens));
    } else {
      tokens.push(positionMarker(nodeTokens, node.loc));
    }

    if (needsParens) {
      tokens.push(operator(')'));
    }

    // If there's an empty line between the node and it's trailing comments then keep it
    const trailingComments = this.getComments(false, node);
    if (trailingComments !== undefined) {
      tokens.push(concat(this.maybeCommentNewlines(
        node,
        trailingComments[0],
        true,
      )));
    }
    tokens.push(concat(this.tokenizeComments(trailingComments)));

    this.printStack.pop();

    return tokens;
  }

  tokenizeJoin(
    nodes: undefined | Array<undefined | AnyNode>,
    parent: AnyNode,
    opts: PrintJoinOptions,
  ): GroupToken {
    const groups: GroupToken['groups'] = [];

    let forceBroken = opts.broken.force || opts.breakOnNewline && n.isMultiLine(
      parent,
    );

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
          if (!isLastNode && opts.newline && !hasNextTrailingCommentOnSameLine) {
            afterBroken.push(newline);
          }

          const newlines = this.maybeInsertExtraStatementNewlines(node, nextNode);

          groups.push({
            tokens: this.tokenize(node, parent),
            afterBroken: [concat(afterBroken), concat(newlines)],
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

  tokenizeCommaList(
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
    return this.tokenizeJoin(items, parent, {
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

  tokenizeStatementList(
    nodes: undefined | Array<undefined | AnyNode>,
    parent: AnyNode,
    shouldIndent: boolean = false,
  ): Tokens {
    const tokens: Tokens = [];
    if (nodes === undefined || nodes.length === 0) {
      return tokens;
    }

    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      if (node === undefined) {
        continue;
      }

      tokens.push(concat(this.tokenize(node, parent)));

      const {
        nextNode,
        hasNextTrailingCommentOnSameLine,
      } = checkTrailingCommentsSameLine(node, nodes[i + 1]);

      if (!hasNextTrailingCommentOnSameLine) {
        tokens.push(newline);
      }

      tokens.push(concat(this.maybeInsertExtraStatementNewlines(node, nextNode)));
    }

    if (shouldIndent) {
      return [newline, indent(tokens), newline];
    }

    return tokens;
  }

  tokenizeTypeColon(node: undefined | AnyNode, parent: AnyNode): Tokens {
    if (node === undefined) {
      return [];
    } else {
      return [operator(':'), space, concat(this.tokenize(node, parent))];
    }
  }

  tokenizeInnerComments(node: AnyNode): Tokens {
    const {innerComments} = node;
    if (innerComments === undefined) {
      return [];
    }

    const tokens: Tokens = [];

    if (n.getLinesBetween(node, innerComments[0])) {
      tokens.push(newline);
    }

    tokens.push(concat(this.tokenizeComments(innerComments)));

    return tokens;
  }

  tokenizeComments(comments: undefined | Array<AnyComment>): Tokens {
    if (!comments || !comments.length) {
      return [];
    }

    const tokens: Tokens = [];

    for (let i = 0; i < comments.length; i++) {
      const comment = comments[i];
      tokens.push(concat(this.tokenizeComment(comment)));

      const nextComment = comments[i + 1];
      if (nextComment !== undefined) {
        tokens.push(
          concat(this.maybeCommentNewlines(comment, nextComment, true)),
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
      return !this.hasTokenizedComment(comment);
    });
  }

  hasTokenizedComment(comment: undefined | AnyComment): boolean {
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

  tokenizeComment(node: AnyComment): Tokens {
    if (this.hasTokenizedComment(node)) {
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
    if (node.type === 'CommentLine' || comment.type === 'CommentLine' &&
        !trailing) {
      lines.shift();
    }

    const tokens: Tokens = [];

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
