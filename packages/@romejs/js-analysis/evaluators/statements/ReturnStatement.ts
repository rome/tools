/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope, FunctionScope} from '../../scopes';
import {ReturnStatement, returnStatement, AnyNode} from '@romejs/js-ast';

export default function ReturnStatement(node: AnyNode, scope: Scope) {
  node = returnStatement.assert(node);
  const funcScope = scope.find(FunctionScope);
  if (node.argument === undefined) {
    // TODO connect to undefined
  } else {
    const type = scope.evaluate(node.argument);
    funcScope.meta.returnType.shouldMatch(type);
  }
}
