/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSImportType, tsImportType} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, word} from '../../tokens';

export default function TSImportType(builder: Builder, node: AnyNode): Tokens {
  node = tsImportType.assert(node);

  let tokens: Tokens = [
    word('import'),
    operator('('),
    ...builder.tokenize(node.argument, node),
    operator(')'),
  ];

  if (node.qualifier) {
    tokens = [
      ...tokens,
      operator('.'),
      ...builder.tokenize(node.qualifier, node),
    ];
  }

  if (node.typeParameters) {
    tokens = [...tokens, ...builder.tokenize(node.typeParameters, node)];
  }

  return tokens;
}
