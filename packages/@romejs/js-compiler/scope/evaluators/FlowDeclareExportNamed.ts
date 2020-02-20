/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {ExportDefaultDeclaration, AnyNode} from '@romejs/js-ast';

export default {
  creator: false,
  build(node: ExportDefaultDeclaration, parent: AnyNode, scope: Scope) {
    const {declaration} = node;
    const newScope = scope.evaluate(declaration, node);
    if (declaration !== undefined) {
      if (declaration.type === 'FlowDeclareOpaqueType') {
        newScope.getBindingAssert(declaration.id.name).setExported(true);
      }
    }
    return newScope;
  },
};
