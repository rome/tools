/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowInterfaceExtends,
  flowInterfaceExtends,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceExtends(node: AnyNode, scope: Scope) {
  node = flowInterfaceExtends.assert(node);

  return scope.evaluate(node.id);
}
