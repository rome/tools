/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypePredicate, tsTypePredicate, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, space, word} from '../../tokens';

export default function TSTypePredicate(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsTypePredicate.assert(node);

  let tokens: Tokens = [];

  if (node.asserts) {
    tokens = [word('asserts'), space];
  }

  tokens = [...tokens, ...generator.print(node.parameterName, node)];

  if (node.typeAnnotation) {
    return [
      ...tokens,
      space,
      word('is'),
      space,
      ...generator.print(node.typeAnnotation, node),
    ];
  } else {
    return tokens;
  }
}
