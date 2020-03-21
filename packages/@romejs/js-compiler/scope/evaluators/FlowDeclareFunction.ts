/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {TypeBinding} from '@romejs/js-compiler';
import {FlowDeclareFunction, AnyNode} from '@romejs/js-ast';

export default {
  creator: false,
  build(node: FlowDeclareFunction, parent: AnyNode, scope: Scope) {
    scope.addBinding(new TypeBinding({
      node: node.id,
      name: node.id.name,
      scope,
    }, node, 'function'));
  },
};
