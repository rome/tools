/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word} from '../../tokens';
import {
  ExportLocalSpecifier,
  exportLocalSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportLocalSpecifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node =
    node.type === 'ExportExternalSpecifier'
      ? node
      : exportLocalSpecifier.assert(node);

  const tokens = generator.print(node.local, node);

  if (node.local.name === node.exported.name) {
    return tokens;
  } else {
    return [
      ...tokens,
      space,
      word('as'),
      space,
      ...generator.print(node.exported, node),
    ];
  }
}
