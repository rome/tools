/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  VariableDeclaration,
  variableDeclaration,
} from '@romejs/js-ast';
import {word, space} from '@romejs/js-generator/tokens';

export default function VariableDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = variableDeclaration.assert(node);

  return [
    word(node.kind),
    space,
    generator.printCommaList(node.declarations, node, {
      indent: false,
    }),
  ];
}
