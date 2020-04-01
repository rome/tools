/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSExpressionWithTypeArguments,
  tsExpressionWithTypeArguments,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens} from '../../tokens';

export default function TSExpressionWithTypeArguments(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsExpressionWithTypeArguments.assert(node);

  return [
    ...generator.print(node.expression, node),
    ...generator.print(node.typeParameters, node),
  ];
}
