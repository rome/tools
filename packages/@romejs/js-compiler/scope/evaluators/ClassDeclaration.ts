/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {ClassBinding} from '@romejs/js-compiler';
import {ClassDeclaration, AnyNode} from '@romejs/js-ast';
import ClassExpression from './ClassExpression';

export default {
  creator: false,
  build(node: ClassDeclaration, parent: AnyNode, scope: Scope) {
    if (node.id !== undefined) {
      scope.addBinding(new ClassBinding({
        name: node.id.name,
        node: node.id,
        scope,
      }));
    }
    return ClassExpression.build(node, parent, scope);
  },
};
