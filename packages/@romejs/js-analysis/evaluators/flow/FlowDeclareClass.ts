/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {FlowDeclareClass, flowDeclareClass, AnyNode} from '@romejs/js-ast';
import ClassT from '../../types/ClassT';

export default function FlowDeclareClass(node: AnyNode, scope: Scope) {
  node = flowDeclareClass.assert(node);
  const bodyScope = scope.fork();
  if (node.typeParameters) {
    bodyScope.evaluate(node.typeParameters);
  }

  const calls = [];
  const instances = [];
  const statics = [];

  for (const propNode of node.body.properties) {
    const prop = bodyScope.evaluate(propNode);
    if (
      propNode.type !== 'FlowObjectTypeSpreadProperty' &&
      propNode.static === true
    ) {
      statics.push(prop);
    } else if (propNode.type === 'FlowObjectTypeCallProperty') {
      calls.push(scope.evaluate(propNode));
    } else {
      instances.push(prop);
    }
  }

  let xtends = undefined;
  if (node.extends.length > 0) {
    xtends = scope.evaluate(node.extends[0]);
  }

  const type = new ClassT(bodyScope, node.id, {
    _constructor: undefined,
    instances,
    statics,
    extends: xtends,
    calls,
  });
  scope.addBinding(node.id.name, type);
  type.setHuman(node.id.name);
  return type;
}
