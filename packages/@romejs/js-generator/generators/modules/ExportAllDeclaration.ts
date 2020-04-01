/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space, operator} from '../../tokens';
import {
  ExportAllDeclaration,
  exportAllDeclaration,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportAllDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = exportAllDeclaration.assert(node);

  return [
    word('export'),
    space,
    operator('*'),
    space,
    word('from'),
    space,
    ...generator.print(node.source, node),
    operator(';'),
  ];
}
