/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  AssignmentExpression,
  assignmentExpression,
} from '@romejs/js-ast';
import * as n from '../../node/index';

export default function AssignmentExpression(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node =
    node.type === 'BinaryExpression' || node.type === 'LogicalExpression'
      ? node
      : assignmentExpression.assert(node);

  // Somewhere inside a for statement `init` node but doesn't usually
  // needs a paren except for `in` expressions: `for (a in b ? a : b;;)`
  const parens =
    generator.inForStatementInitCounter > 0 &&
    node.operator === 'in' &&
    !n.needsParens(node, parent, []);
  const {left, right} = node;

  if (parens) {
    generator.token('(');
  }

  generator.print(left, node);

  generator.space();
  if (node.operator === 'in' || node.operator === 'instanceof') {
    generator.word(node.operator);
  } else {
    generator.token(node.operator);
  }
  generator.space();

  const isMultiLine =
    left.loc !== undefined &&
    right.loc !== undefined &&
    right.loc.start.line > left.loc.end.line;
  if (isMultiLine) {
    generator.newline();
  }

  generator.print(right, node);

  if (parens) {
    generator.token(')');
  }
}
