/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSCallSignatureDeclaration,
  tsCallSignatureDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator, space} from '../../tokens';

export default function TSCallSignatureDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsCallSignatureDeclaration.assert(node);

  let tokens: Tokens = generator.print(node.meta, node);

  if (node.typeAnnotation !== undefined) {
    tokens = [
      ...tokens,
      operator(':'),
      space,
      ...generator.print(node.typeAnnotation, node),
    ];
  }

  return [...tokens, operator(';')];
}
