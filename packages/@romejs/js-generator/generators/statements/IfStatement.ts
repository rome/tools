/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {IfStatement, ifStatement, AnyNode} from '@romejs/js-ast';
import {isStatement} from '@romejs/js-ast-utils';
export default function IfStatement(generator: Generator, node: AnyNode) {
  node = ifStatement.assert(node);
  ifStatement.assert(node);
  generator.word('if');
  generator.space();
  generator.token('(');
  generator.multiline(node.test, (multiline, test) => {
    generator.print(test, node);
  }, {indent: true, indentTrailingNewline: true});
  generator.token(')');
  generator.space();

  let needsBlock = false;
  if (node.alternate !== undefined) {
    needsBlock = getLastStatement(node.consequent).type === 'IfStatement';
  }
  if (needsBlock) {
    generator.token('{');
    generator.forceNewline();
    generator.indent();
  }

  generator.print(node.consequent, node);

  if (needsBlock) {
    generator.dedent();
    generator.forceNewline();
    generator.token('}');
  }

  if (node.alternate) {
    if (generator.buf.endsWith('}')) {
      generator.space();
    }
    generator.word('else');
    generator.space();
    generator.print(node.alternate, node);
  }

  generator.forceNewline();
}

// Recursively get the last statement.
function getLastStatement(statement: AnyNode): AnyNode {
  if (
    (statement.type === 'WithStatement' || statement.type === 'WhileStatement' ||
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
