/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSParenthesizedType,
  tsParenthesizedType,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSParenthesizedType(
  generator: Generator,
  node: AnyNode,
) {
  node = tsParenthesizedType.assert(node);
  generator.token('(');
  generator.print(node.typeAnnotation, node);
  generator.token(')');
}
