/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExportAllDeclaration} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';

export default function ExportAllDeclaration(
  builder: Builder,
  node: ExportAllDeclaration,
): Token {
  const tokens: Array<Token> = ['export', space];

  if (node.exportKind === 'type') {
    if (!builder.options.typeAnnotations) {
      return '';
    }

    tokens.push('type', space);
  }

  tokens.push(
    '*',
    space,
    'from',
    space,
    builder.tokenize(node.source, node),
    ';',
  );

  return concat(tokens);
}
