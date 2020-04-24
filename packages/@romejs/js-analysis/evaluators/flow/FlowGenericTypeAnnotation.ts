/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowGenericTypeAnnotation,
  FlowQualifiedTypeIdentifier,
  Identifier,
  ReferenceIdentifier,
  flowGenericTypeAnnotation,
} from '@romejs/js-ast';
import GenericT from '../../types/GenericT';

function getName(
  node: Identifier | ReferenceIdentifier | FlowQualifiedTypeIdentifier,
): string {
  if (node.type === 'Identifier' || node.type === 'ReferenceIdentifier') {
    return node.name;
  } else {
    return `${getName(node.id)}.${getName(node.qualification)}`;
  }
}

export default function FlowGenericTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowGenericTypeAnnotation.assert(node);

  //if (node.typeParameters) {
  //  // TODO execute type params

  //  return new InstanceT(scope, node, node.id.name, scope.evaluate(node.id), []);

  //} else {
  return new GenericT(scope, node, getName(node.id), scope.evaluate(node.id));
  //}
}
