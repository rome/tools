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
  BinaryExpression,
  LogicalExpression,
  assignmentExpression,
} from '@romejs/js-ast';
import * as n from '../../node/index';

type OurNode = AssignmentExpression | BinaryExpression | LogicalExpression;

export default function AssignmentExpression(
  generator: Generator,
  _node: AnyNode,
  parent: AnyNode,
) {
  const node: OurNode = _node.type === 'BinaryExpression' || _node.type ===
  'LogicalExpression' ? _node : assignmentExpression.assert(_node);

  // Somewhere inside a for statement `init` node but doesn't usually

  // needs a paren except for `in` expressions: `for (a in b ? a : b;;)`
  const needsExtraParens = generator.inForStatementInitCounter > 0 &&
    node.operator === 'in' && !n.needsParens(node, parent, []);

  if (needsExtraParens) {
    generator.token('(');
  }

  generator.multiline(node, (multiline, node) => {
    const shouldIndent = multiline && (node.operator === '&&' ||
    node.operator === '=');

    generator.print(node.left, node);

    generator.space();
    if (node.operator === 'in' || node.operator === 'instanceof') {
      generator.word(node.operator);
    } else {
      generator.token(node.operator);
    }

    generator.spaceOrNewline(multiline);

    if (shouldIndent) {
      generator.indent();
    }

    generator.print(node.right, node);

    if (shouldIndent) {
      generator.dedent();
    }
  }, {conditions: ['any-line-exceeds']});

  if (needsExtraParens) {
    generator.token(')');
  }
}
