/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowDeclareVariable,
  flowDeclareVariable,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareVariable(node: AnyNode, scope: Scope) {
  node = flowDeclareVariable.assert(node);

  const {id} = node;
  if (id.meta === undefined) {
    throw new Error('TODO');
  }

  const type = scope.evaluate(id.meta.typeAnnotation);
  scope.addBinding(id.name, type);
  return type;
}
