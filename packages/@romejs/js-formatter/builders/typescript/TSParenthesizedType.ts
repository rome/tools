/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSParenthesizedType,
  tsParenthesizedType,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSParenthesizedType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsParenthesizedType.assert(node);

  return [
    operator('('),
    ...builder.tokenize(node.typeAnnotation, node),
    operator(')'),
  ];
}
