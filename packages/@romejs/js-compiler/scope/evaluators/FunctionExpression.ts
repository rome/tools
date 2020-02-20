/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {LetBinding} from '@romejs/js-compiler';
import {addFunctionBindings} from '../utils';
import {FunctionExpression, AnyNode} from '@romejs/js-ast';

export default {
  creator: true,
  build(node: FunctionExpression, parent: AnyNode, scope: Scope) {
    const newScope = scope.fork('function', node);
    if (node.id !== undefined) {
      newScope.addBinding(
        new LetBinding({
          node: node.id,
          name: node.id.name,
          scope,
        }),
      );
    }
    addFunctionBindings(newScope, node);
    return newScope;
  },
};
