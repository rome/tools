/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSExternalModuleReference,
  tsExternalModuleReference,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSExternalModuleReference(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsExternalModuleReference.assert(node);

  return [
    operator('require('),
    ...builder.tokenize(node.expression, node),
    operator(')'),
  ];
}
