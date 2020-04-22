/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {addFunctionBindings} from '../utils';
import {AnyNode, ArrowFunctionExpression} from '@romejs/js-ast';

export default {
  creator: true,
  build(node: ArrowFunctionExpression, parent: AnyNode, scope: Scope) {
    const newScope = scope.fork('function', node);
    addFunctionBindings(newScope, node, false);
    return newScope;
  },
};
