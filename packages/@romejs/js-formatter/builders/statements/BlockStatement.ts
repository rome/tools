/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, hardline, indent} from '../../tokens';
import {AnyNode, BlockStatement} from '@romejs/js-ast';

export default function BlockStatement(
  builder: Builder,
  node: BlockStatement,
  parent: AnyNode,
): Token {
  const hasComments =
    node.innerComments !== undefined && node.innerComments.length > 0;
  const hasContents = node.body !== undefined && node.body.length > 0;
  const hasDirectives =
    node.directives !== undefined && node.directives.length > 0;

  if (
    !hasComments &&
    !hasContents &&
    !hasDirectives &&
    (parent.type === 'ArrowFunctionExpression' ||
    parent.type === 'ClassMethod' ||
    parent.type === 'ClassPrivateMethod' ||
    parent.type === 'DoWhileStatement' ||
    parent.type === 'ForInStatement' ||
    parent.type === 'ForOfStatement' ||
    parent.type === 'ForStatement' ||
    parent.type === 'FunctionDeclaration' ||
    parent.type === 'FunctionExpression' ||
    parent.type === 'ObjectMethod' ||
    parent.type === 'SwitchStatement' ||
    parent.type === 'WhileStatement')
  ) {
    return '{}';
  }

  const tokens: Array<Token> = ['{'];

  if (hasDirectives) {
    for (const directive of node.directives!) {
      tokens.push(indent(concat([hardline, builder.tokenize(directive, node)])));
    }
  }

  if (hasContents) {
    tokens.push(
      indent(concat([hardline, builder.tokenizeStatementList(node.body, node)])),
    );
  }

  if (hasComments) {
    tokens.push(builder.tokenizeInnerComments(node, true));
  }

  tokens.push(hardline, '}');

  return concat(tokens);
}
