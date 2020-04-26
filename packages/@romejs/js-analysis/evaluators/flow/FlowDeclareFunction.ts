/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowDeclareFunction,
  flowDeclareFunction,
} from '@romejs/js-ast';

export default function FlowDeclareFunction(node: AnyNode, scope: Scope) {
  node = flowDeclareFunction.assert(node);

  return scope.addBinding(
    node.id.name,
    scope.evaluate(node.id.meta.typeAnnotation),
  );
}
