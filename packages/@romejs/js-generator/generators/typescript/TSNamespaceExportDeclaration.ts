/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSNamespaceExportDeclaration,
  tsNamespaceExportDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, word, space} from '../../tokens';

export default function TSNamespaceExportDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsNamespaceExportDeclaration.assert(node);

  return [
    word('export'),
    space,
    word('as'),
    space,
    word('namespace'),
    space,
    ...generator.print(node.id, node),
  ];
}
