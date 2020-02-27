/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {FlowTypeParameter, flowTypeParameter, AnyNode} from '@romejs/js-ast';
import AnyT from '../../types/AnyT';

export default function FlowTypeParameter(node: AnyNode, scope: Scope) {
  node = flowTypeParameter.assert(node);
  const type = new AnyT(scope, node);
  scope.addBinding(node.name, type);
  return type;
}
