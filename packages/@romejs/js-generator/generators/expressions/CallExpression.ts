/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {CallExpression, callExpression, AnyNode} from '@romejs/js-ast';

export default function CallExpression(generator: Generator, node: AnyNode) {
  node =
    node.type === 'OptionalCallExpression' || node.type === 'NewExpression'
      ? node
      : callExpression.assert(node);

  generator.multiline(node, (multiline, node) => {
    generator.print(node.callee, node);
    generator.print(node.typeArguments, node);

    const startLine = generator.buf.position.line;
    const startIndent = generator.currentLineIndentLevel;

    if (node.type === 'OptionalCallExpression') {
      generator.token('?');
    }

    generator.token('(');
    generator.printCommaList(node.arguments, node, {
      multiline,
      trailing: true,
    });

    // TODO add newline if we've added a line and are on a different indentation level
    const endLine = generator.buf.position.line;
    const endIndent = generator.currentLineIndentLevel;
    if (startLine !== endLine && startIndent !== endIndent) {
      generator.newline();
    }

    generator.token(')');
  });
}
