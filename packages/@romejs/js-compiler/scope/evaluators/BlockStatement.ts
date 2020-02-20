/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {BlockStatement, AnyNode} from '@romejs/js-ast';

export default {
  creator: true,
  build(node: BlockStatement, parent: AnyNode, scope: Scope) {
    const newScope = scope.fork('block', node);
    for (const child of node.body) {
      newScope.evaluate(child, node);
    }
    return newScope;
  },
};
