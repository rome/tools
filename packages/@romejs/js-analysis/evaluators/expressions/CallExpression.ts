/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {AnyNode, CallExpression, callExpression} from '@romejs/js-ast';
import CallT from '../../types/CallT';

export default function CallExpression(node: AnyNode, scope: Scope) {
  node = callExpression.assert(node);

  return new CallT(scope, node, scope.evaluate(node.callee), node.arguments.map(
    (
      arg,
    ) => {
      return scope.evaluate(arg);
    },
  ));
}
