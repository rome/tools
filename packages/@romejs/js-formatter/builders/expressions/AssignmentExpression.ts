/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, word, breakGroup, space} from '../../tokens';
import {
  AnyNode,
  AssignmentExpression,
  BinaryExpression,
  LogicalExpression,
  assignmentExpression,
} from '@romejs/js-ast';
import * as n from '../../node/index';

type OurNode = AssignmentExpression | BinaryExpression | LogicalExpression;

export default function AssignmentExpression(
  builder: Builder,
  _node: AnyNode,
  parent: AnyNode,
): Tokens {
  const node: OurNode = _node.type === 'BinaryExpression' || _node.type ===
    'LogicalExpression' ? _node : assignmentExpression.assert(_node);

  let tokens: Tokens = [];

  // Somewhere inside a for statement `init` node but doesn't usually
  // needs a paren except for `in` expressions: `for (a in b ? a : b;;)`
  const needsExtraParens = builder.inForStatementInitCounter > 0 &&
      node.operator ===
      'in' && !n.needsParens(node, parent, []);

  if (needsExtraParens) {
    tokens.push(operator('('));
  }

  const left = builder.tokenize(node.left, node);

  let sep;
  if (node.operator === 'in' || node.operator === 'instanceof') {
    sep = word(node.operator);
  } else {
    sep = operator(node.operator);
  }

  const right = builder.tokenize(node.right, node);

  tokens.push(breakGroup([[...left, space, sep], right]));

  if (needsExtraParens) {
    tokens.push(operator(')'));
  }

  return tokens;
}
