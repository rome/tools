/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {TypeBinding} from '@romejs/js-compiler';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {AnyNode, FlowTypeParameterDeclaration} from '@romejs/js-ast';

export default {
  creator: false,
  build(node: FlowTypeParameterDeclaration, parent: AnyNode, scope: Scope) {
    for (const id of getBindingIdentifiers(node)) {
      scope.addBinding(
        new TypeBinding(
          {
            node: id,
            name: id.name,
            scope,
          },
          node,
          'parameter',
        ),
      );
    }
  },
};
