/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word, concat} from '../../tokens';
import {
  ExportLocalSpecifier,
  exportLocalSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportLocalSpecifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'ExportExternalSpecifier'
    ? node
    : exportLocalSpecifier.assert(node);

  const tokens = builder.tokenize(node.local, node);

  if (node.local.name === node.exported.name) {
    return tokens;
  } else {
    return [
      concat(tokens),
      space,
      word('as'),
      space,
      concat(builder.tokenize(node.exported, node)),
    ];
  }
}
