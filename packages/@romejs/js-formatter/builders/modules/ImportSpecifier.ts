/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space} from '../../tokens';
import {ImportSpecifier, importSpecifier, AnyNode} from '@romejs/js-ast';

export default function ImportSpecifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = importSpecifier.assert(node);

  let tokens: Tokens = [];

  if (node.local.importKind === 'type' || node.local.importKind === 'typeof') {
    tokens = [word(node.local.importKind), space];
  }

  tokens = [...tokens, ...builder.tokenize(node.imported, node)];

  if (node.local.name.name !== node.imported.name) {
    tokens = [
      ...tokens,
      space,
      word('as'),
      space,
      ...builder.tokenize(node.local.name, node),
    ];
  }

  return tokens;
}
