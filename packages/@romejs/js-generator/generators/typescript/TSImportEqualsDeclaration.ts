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
import {Generator} from '@romejs/js-generator';
import {Tokens, space, word, operator} from '../../tokens';

export default function TSImportEqualsDeclaration(
  generator: Generator,
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
    ...generator.print(node.id, node),
    space,
    operator('='),
    space,
    ...generator.print(node.moduleReference, node),
    operator(';'),
  ];
}
