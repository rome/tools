/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  AmbiguousFlowTypeCastExpression,
  ambiguousFlowTypeCastExpression,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function AmbiguousFlowTypeCastExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = ambiguousFlowTypeCastExpression.assert(node);
  throw new Error('unimplemented');
}
