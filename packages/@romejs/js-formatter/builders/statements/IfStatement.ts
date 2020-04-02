/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  Tokens,
  breakGroup,
  indent,
  operator,
  word,
  space,
  newline,
} from '../../tokens';
import {IfStatement, ifStatement, AnyNode} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';

export default function IfStatement(builder: Builder, node: AnyNode): Tokens {
  node = ifStatement.assert(node);

  let tokens: Tokens = [
    word('if'),
    space,
    operator('('),
    breakGroup([builder.tokenize(node.test, node)], true),
    operator(')'),
    space,
  ];

  let needsBlock = false;
  if (node.alternate !== undefined) {
    needsBlock = getLastStatement(node.consequent).type === 'IfStatement';
  }
  if (needsBlock) {
    tokens = [
      ...tokens,
      operator('{'),
      newline,
      indent(builder.tokenize(node.consequent, node)),
      newline,
      operator('}'),
    ];
  } else {
    tokens = [...tokens, ...builder.tokenize(node.consequent, node)];
  }

  if (node.alternate) {
    tokens = [
      ...tokens,
      space,
      word('else'),
      space,
      ...builder.tokenize(node.alternate, node),
    ];
  }

  return tokens;
}

// Recursively get the last statement.
function getLastStatement(statement: AnyNode): AnyNode {
  let node = statement;
  while ((node.type === 'WithStatement' || node.type === 'WhileStatement' ||
      node.type ===
      'DoWhileStatement' || node.type === 'ForOfStatement' || node.type ===
    'ForInStatement' || node.type === 'ForStatement') && isStatement(node.body)) {
    node = node.body;
  }

  return node;
}
