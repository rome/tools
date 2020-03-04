/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {LetBinding} from '@romejs/js-compiler';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {AnyNode, CatchClause} from '@romejs/js-ast';

export default {
  creator: true,
  build(node: CatchClause, parent: AnyNode, scope: Scope) {
    const newScope = scope.fork('block', node);
    if (node.param !== undefined) {
      for (const id of getBindingIdentifiers(node.param)) {
        // TODO maybe add a `catch` binding type?
        newScope.addBinding(
          new LetBinding({
            node: id,
            name: id.name,
            scope,
          }),
        );
      }
    }
    return newScope;
  },
};
