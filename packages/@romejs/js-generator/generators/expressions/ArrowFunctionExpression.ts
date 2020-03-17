/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ArrowFunctionExpression,
  arrowFunctionExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ArrowFunctionExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = arrowFunctionExpression.assert(node);

  if (node.head.async === true) {
    generator.word('async');
    generator.space();
  }

  generator.print(node.head, node);

  generator.space();
  generator.token('=>');
  generator.space();

  const {body} = node;
  if (body.type === 'BlockStatement') {
    generator.print(body, node);
  } else {
    generator.multiline(
      node,
      multiline => {
        if (multiline) {
          generator.newline();
          generator.indent();
        }

        generator.print(body, node);

        if (multiline) {
          generator.dedent();
        }
      },
      {conditions: ['more-than-one-line']},
    );
  }
}
