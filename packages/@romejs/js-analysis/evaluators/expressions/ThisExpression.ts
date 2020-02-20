/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {ThisScope} from '../../scopes';
import {ThisExpression, thisExpression, AnyNode} from '@romejs/js-ast';
import OpenT from '../../types/OpenT';

export default function ThisExpression(node: AnyNode, scope: Scope) {
  node = thisExpression.assert(node);
  const thisScope = scope.find(ThisScope);
  if (thisScope === undefined) {
    // TODO complain
  } else {
    const type = new OpenT(scope, node);
    type.shouldMatch(thisScope.context);
    return type;
  }
}
