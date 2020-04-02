/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, word} from '../../tokens';
import {
  ExportLocalDeclaration,
  exportLocalDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {isDeclaration} from '@romejs/js-ast-utils';

export default function ExportLocalDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = exportLocalDeclaration.assert(node);

  if (node.exportKind === 'type' && !builder.options.typeAnnotations) {
    return [];
  }

  return [word('export'), space, ..._ExportDeclaration(builder, node)];
}

export function _ExportDeclaration(builder: Builder, node: AnyNode): Tokens {
  node = node.type === 'ExportDefaultDeclaration'
    ? node
    : exportLocalDeclaration.assert(node);

  if (node.declaration) {
    const declar = node.declaration;
    const tokens = builder.tokenize(declar, node);
    if (!isDeclaration(declar)) {
      tokens.push(operator(';'));
    }
    return tokens;
  } else {
    if (node.type !== 'ExportLocalDeclaration') {
      throw new Error('Expected  ExportLocalDeclaration');
    }

    let tokens: Tokens = [];

    if (node.exportKind === 'type') {
      tokens = [word('type'), space];
    }

    const {specifiers} = node;
    if (specifiers === undefined) {
      throw new Error('Expected specifiers since there was no declaration');
    }

    return [
      ...tokens,
      operator('{'),
      builder.tokenizeCommaList(specifiers, node, {
        trailing: true,
      }),
      operator('}'),
      operator(';'),
    ];
  }
}
