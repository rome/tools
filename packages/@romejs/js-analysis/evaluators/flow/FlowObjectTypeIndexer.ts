/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowObjectTypeIndexer,
  flowObjectTypeIndexer,
  AnyNode,
} from '@romejs/js-ast';
import ObjIndexPropT from '../../types/ObjIndexPropT';

export default function FlowObjectTypeIndexer(node: AnyNode, scope: Scope) {
  node = flowObjectTypeIndexer.assert(node);

  return new ObjIndexPropT(
    scope,
    node,
    scope.evaluate(node.key),
    scope.evaluate(node.value),
  );
}
