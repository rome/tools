/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowNullableTypeAnnotation,
  flowNullableTypeAnnotation,
} from '@romejs/js-ast';
import MaybeT from '../../types/MaybeT';

export default function FlowNullableTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowNullableTypeAnnotation.assert(node);
  return new MaybeT(scope, node, scope.evaluate(node.typeAnnotation));
}
