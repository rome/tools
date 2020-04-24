/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from './Scope';
import {REDUCE_SKIP_SUBTREE} from '@romejs/js-compiler';
import {isFunctionNode} from '@romejs/js-ast-utils';
import {Program, AnyFunction} from '@romejs/js-ast';

export function addVarBindings(scope: Scope, topNode: AnyFunction | Program) {
  const {context} = scope.getRootScope();
  scope.setHoistedVars();

  context.reduce(topNode, [
    {
      name: 'scopeVarFunc',
      enter: (path) => {
        const {node, parent} = path;

        if (isFunctionNode(node) && node !== topNode) {
          return REDUCE_SKIP_SUBTREE;
        }

        if (node.type === 'VariableDeclaration' && node.kind === 'var') {
          scope.evaluate(node, parent);
        }

        return node;
      },
    },
  ], {
    scope,
    noScopeCreation: true,
  });
}
