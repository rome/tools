/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSImportEqualsDeclaration,
  tsImportEqualsDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, space, word, operator} from '../../tokens';

export default function TSImportEqualsDeclaration(
  builder: Builder,
  node: AnyNode,
) {
  node = tsImportEqualsDeclaration.assert(node);

  let tokens: Tokens = [];
  if (node.isExport) {
    tokens.push(word('export'));
    tokens.push(space);
  }

  return [
    ...tokens,
    word('import'),
    space,
    ...builder.tokenize(node.id, node),
    space,
    operator('='),
    space,
    ...builder.tokenize(node.moduleReference, node),
    operator(';'),
  ];
}
