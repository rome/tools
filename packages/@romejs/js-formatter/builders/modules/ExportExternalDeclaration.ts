/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  Tokens,
  linkedGroups,
  operator,
  space,
  word,
  concat,
} from '../../tokens';
import {AnyNode, exportExternalDeclaration} from '@romejs/js-ast';
import {printModuleSpecifiers} from './ImportDeclaration';

export default function ExportExternalDeclaration(
  builder: Builder,
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
      concat(tokens),
      concat(printModuleSpecifiers(builder, node)),
      space,
      word('from'),
      space,
      concat(builder.tokenize(node.source, node)),
      operator(';'),
    ]),
  ];
}
