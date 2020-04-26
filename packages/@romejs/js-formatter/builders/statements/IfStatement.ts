/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  Token,
  concat,
  group,
  hardline,
  indent,
  softline,
  space,
} from '../../tokens';
import {AnyNode, IfStatement} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';

export default function IfStatement(builder: Builder, node: IfStatement): Token {
  const tokens: Array<Token> = [
    group(
      concat([
        'if',
        space,
        '(',
        group(
          concat([
            indent(concat([softline, builder.tokenize(node.test, node)])),
            softline,
          ]),
        ),
        ')',
      ]),
    ),
    space,
  ];

  let needsBlock = false;
  if (node.alternate) {
    needsBlock = getLastStatement(node.consequent).type === 'IfStatement';
  }

  if (needsBlock) {
    tokens.push(
      '{',
      indent(concat([hardline, builder.tokenize(node.consequent, node)])),
      hardline,
      '}',
    );
  } else {
    tokens.push(builder.tokenize(node.consequent, node));
  }

  if (node.alternate) {
    tokens.push(space, 'else', space, builder.tokenize(node.alternate, node));
  }

  return concat(tokens);
}

// Recursively get the last statement.
function getLastStatement(statement: AnyNode): AnyNode {
  if (
    (statement.type === 'WithStatement' ||
    statement.type === 'WhileStatement' ||
    statement.type === 'DoWhileStatement' ||
    statement.type === 'ForOfStatement' ||
    statement.type === 'ForInStatement' ||
    statement.type === 'ForStatement') &&
    isStatement(statement.body)
  ) {
    return getLastStatement(statement.body);
  } else {
    return statement;
  }
}
