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
import {Tokens} from '../../tokens';

export default function AmbiguousFlowTypeCastExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = ambiguousFlowTypeCastExpression.assert(node);
  throw new Error('unimplemented');
}
