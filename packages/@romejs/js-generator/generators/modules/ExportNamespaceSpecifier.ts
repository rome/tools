/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space, word} from '../../tokens';
import {
  ExportNamespaceSpecifier,
  exportNamespaceSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportNamespaceSpecifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = exportNamespaceSpecifier.assert(node);

  return [
    operator('*'),
    space,
    word('as'),
    space,
    ...generator.print(node.exported, node),
  ];
}
