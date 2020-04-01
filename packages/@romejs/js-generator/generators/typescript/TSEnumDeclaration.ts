/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumDeclaration, tsEnumDeclaration, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, word, space} from '../../tokens';
import {printTSBraced} from '../utils';

export default function TSEnumDeclaration(
  generator: Generator,
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
    ...generator.print(node.id, node),
    space,
    ...printTSBraced(generator, node, node.members),
  ];
}
