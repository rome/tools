/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowTypeCastExpression,
  flowTypeCastExpression,
} from '@romejs/js-ast';
import ExhaustiveT from '../../types/ExhaustiveT';

export default function FlowTypeCastExpression(node: AnyNode, scope: Scope) {
  node = flowTypeCastExpression.assert(node);
  const expressionType = scope.evaluate(node.expression);
  const assertedType = scope.evaluate(node.typeAnnotation);
  new ExhaustiveT(scope, node, expressionType, assertedType);
  return assertedType;
}
