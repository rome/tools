/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSImportType, tsImportType, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, word, operator} from '../../tokens';

export default function TSImportType(builder: Builder, node: AnyNode): Tokens {
  node = tsImportType.assert(node);

  let tokens: Tokens = [
    word('import'),
    operator('('),
    ...builder.print(node.argument, node),
    operator(')'),
  ];

  if (node.qualifier) {
    tokens = [...tokens, operator('.'), ...builder.print(node.qualifier, node)];
  }

  if (node.typeParameters) {
    tokens = [...tokens, ...builder.print(node.typeParameters, node)];
  }

  return tokens;
}
