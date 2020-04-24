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
  concat,
  indent,
  newline,
  operator,
  space,
  word,
} from '../../tokens';
import {AnyNode, ifStatement} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';

export default function IfStatement(builder: Builder, node: AnyNode): Tokens {
  node = ifStatement.assert(node);

  const tokens: Tokens = [
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
    tokens.push(operator('{'), newline, indent(builder.tokenize(
      node.consequent,
      node,
    )), newline, operator('}'));
  } else {
    tokens.push(concat(builder.tokenize(node.consequent, node)));
  }

  if (node.alternate) {
    tokens.push(space, word('else'), space, concat(builder.tokenize(
      node.alternate,
      node,
    )));
  }

  return tokens;
}

// Recursively get the last statement.
function getLastStatement(statement: AnyNode): AnyNode {
  if ((statement.type === 'WithStatement' || statement.type === 'WhileStatement' ||
              statement.type === 'DoWhileStatement' ||
            statement.type === 'ForOfStatement' ||
          statement.type === 'ForInStatement' ||
        statement.type === 'ForStatement') &&
      isStatement(statement.body)) {
    return getLastStatement(statement.body);
  } else {
    return statement;
  }
}
