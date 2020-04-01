/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, linkedGroups, operator, space, word} from '../../tokens';
import {
  AnyNode,
  ExportExternalDeclaration,
  exportExternalDeclaration,
} from '@romejs/js-ast';
import {printModuleSpecifiers} from './ImportDeclaration';

export default function ExportExternalDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = exportExternalDeclaration.assert(node);

  const tokens: Tokens = [word('export'), space];

  if (node.exportKind === 'type') {
    tokens.push(word('type'));
    tokens.push(space);
  }

  return [
    linkedGroups([
      ...tokens,
      ...printModuleSpecifiers(generator, node),
      space,
      word('from'),
      space,
      ...generator.print(node.source, node),
      operator(';'),
    ]),
  ];
}
