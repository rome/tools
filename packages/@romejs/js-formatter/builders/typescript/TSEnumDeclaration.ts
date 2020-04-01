/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumDeclaration, tsEnumDeclaration, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, word, space} from '../../tokens';
import {printTSBraced} from '../utils';

export default function TSEnumDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsEnumDeclaration.assert(node);

  let tokens: Tokens = [];

  if (node.declare) {
    tokens = [word('declare'), space];
  }

  if (node.const) {
    tokens = [...tokens, word('const'), space];
  }

  return [
    ...tokens,
    word('enum'),
    space,
    ...builder.tokenize(node.id, node),
    space,
    ...printTSBraced(builder, node, node.members),
  ];
}
