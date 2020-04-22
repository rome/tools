/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AmbiguousFlowTypeCastExpression,
  AnyNode,
  ambiguousFlowTypeCastExpression,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens} from '../../tokens';

export default function AmbiguousFlowTypeCastExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = ambiguousFlowTypeCastExpression.assert(node);
  throw new Error('unimplemented');
}
