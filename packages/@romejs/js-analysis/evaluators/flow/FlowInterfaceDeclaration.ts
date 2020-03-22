/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowInterfaceDeclaration,
  flowInterfaceDeclaration,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceDeclaration(node: AnyNode, scope: Scope) {
  node = node.type === 'FlowDeclareInterface'
    ? node : flowInterfaceDeclaration.assert(node);

  const typeScope = scope.fork();
  if (node.typeParameters) {
    typeScope.evaluate(node.typeParameters);
  }

  // TODO extends
  const body = typeScope.evaluate(node.body);
  scope.addBinding(node.id.name, body);
  return body;
}
