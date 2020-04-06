/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, concat} from '../../tokens';
import {ImportSpecifier, importSpecifier, AnyNode} from '@romejs/js-ast';

export default function ImportSpecifier(builder: Builder, node: AnyNode): Tokens {
  node = importSpecifier.assert(node);

  const tokens: Tokens = [];

  if (node.local.importKind === 'type' || node.local.importKind === 'typeof') {
    tokens.push(word(node.local.importKind), space);
  }

  tokens.push(concat(builder.tokenize(node.imported, node)));

  if (node.local.name.name !== node.imported.name) {
    tokens.push(space, word('as'), space, concat(builder.tokenize(
      node.local.name,
      node,
    )));
  }

  return tokens;
}
