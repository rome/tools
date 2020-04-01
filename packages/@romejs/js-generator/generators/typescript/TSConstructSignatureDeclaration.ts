/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSConstructSignatureDeclaration,
  tsConstructSignatureDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, word, space, operator} from '../../tokens';

export default function TSConstructSignatureDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsConstructSignatureDeclaration.assert(node);

  let tokens: Tokens = [
    word('new'),
    space,
    ...generator.print(node.meta, node),
  ];

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
