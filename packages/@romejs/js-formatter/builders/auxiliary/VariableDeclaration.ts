/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  AnyNode,
  VariableDeclaration,
  variableDeclaration,
} from '@romejs/js-ast';
import {word, space} from '@romejs/js-formatter/tokens';

export default function VariableDeclaration(builder: Builder, node: AnyNode) {
  node = variableDeclaration.assert(node);

  return [
    word(node.kind),
    space,
    builder.tokenizeCommaList(node.declarations, node, {
      indent: false,
    }),
  ];
}
