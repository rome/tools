/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {assignmentExpression, AnyNode} from '@romejs/js-ast';
import SideEffectT from '../../types/SideEffectT';

export default function AssignmentExpression(node: AnyNode, scope: Scope) {
  node = assignmentExpression.assert(node);

  const {left, right, operator} = node;

  if (operator === '=') {
    const rightType = scope.evaluate(right);
    const leftType = scope.evaluate(left);
    leftType.shouldMatch(rightType);
    return new SideEffectT(scope, node, rightType);
  } else {
    // TODO!
    return undefined;
  }
}
