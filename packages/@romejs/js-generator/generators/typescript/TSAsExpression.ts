/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSAsExpression, tsAsExpression, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, space, word} from '../../tokens';

export default function TSAsExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsAsExpression.assert(node);

  if (generator.options.typeAnnotations) {
    return [
      ...generator.print(node.expression, node),
      space,
      word('as'),
      space,
      ...generator.print(node.typeAnnotation, node),
    ];
  } else {
    return generator.print(node.expression, node);
  }
}
