/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowObjectTypeAnnotation,
  flowObjectTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';
import ObjT from '../../types/ObjT';
import T from '../../types/T';

export default function FlowObjectTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowObjectTypeAnnotation.assert(node);

  const props: Array<T> = [];
  const calls: Array<T> = [];

  for (const prop of node.properties) {
    props.push(scope.evaluate(prop));
  }

  return new ObjT(scope, node, {
    props,
    proto: scope.intrinsics.ObjectPrototype,
    calls,
  });
}
