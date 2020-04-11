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
import {Builder} from '@romejs/js-formatter';
import {Tokens, word, space, operator} from '../../tokens';

export default function TSConstructSignatureDeclaration(
  builder: Builder,
  node: AnyNode,
) {
  node = tsConstructSignatureDeclaration.assert(node);

  let tokens: Tokens = [
    word('new'),
    space,
    ...builder.tokenize(node.meta, node),
  ];

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
