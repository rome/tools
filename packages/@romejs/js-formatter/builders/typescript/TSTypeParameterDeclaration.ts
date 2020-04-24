/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSTypeParameterDeclaration,
  tsTypeParameterDeclaration,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSTypeParameterDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsTypeParameterDeclaration.assert(node);

  return [
    operator('<'),
    builder.tokenizeCommaList(node.params, node),
    operator('>'),
  ];
}
