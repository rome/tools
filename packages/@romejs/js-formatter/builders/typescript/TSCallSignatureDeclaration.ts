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
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space} from '../../tokens';

export default function TSCallSignatureDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsCallSignatureDeclaration.assert(node);

  let tokens: Tokens = builder.tokenize(node.meta, node);

  if (node.typeAnnotation !== undefined) {
    tokens = [
      ...tokens,
      operator(':'),
      space,
      ...builder.tokenize(node.typeAnnotation, node),
    ];
  }

  return [...tokens, operator(';')];
}
