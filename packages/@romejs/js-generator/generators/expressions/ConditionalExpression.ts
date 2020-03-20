/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ConditionalExpression,
  conditionalExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ConditionalExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = conditionalExpression.assert(node);

  generator.multiline(node, (multiline, node) => {
    generator.print(node.test, node);

    if (multiline) {
      generator.newline();
      generator.indent();
    } else {
      generator.space();
    }

    generator.token('?');
    generator.space();
    generator.print(node.consequent, node);
    generator.space();
    generator.token(':');
    generator.space();
    generator.print(node.alternate, node);

    if (multiline) {
      generator.dedent();
    }
  });
}
