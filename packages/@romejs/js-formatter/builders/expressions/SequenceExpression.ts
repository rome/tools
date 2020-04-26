/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, SequenceExpression} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, group, indent, join, lineOrSpace} from '../../tokens';

export default function SequenceExpression(
  builder: Builder,
  node: SequenceExpression,
  parent: AnyNode,
): Token {
  if (
    parent.type === 'ExpressionStatement' ||
    parent.type === 'ForStatement' ||
    parent.type === 'SequenceExpression'
  ) {
    // Indent expressions after the first to improve the readability
    return group(
      concat(
        node.expressions.map((expr, i) =>
          i === 0
            ? builder.tokenize(expr, node)
            : concat([
                ',',
                indent(concat([lineOrSpace, builder.tokenize(expr, node)])),
              ])
        ),
      ),
    );
  } else {
    return group(
      join(
        concat([',', lineOrSpace]),
        node.expressions.map((expr) => builder.tokenize(expr, node)),
      ),
    );
  }
}
