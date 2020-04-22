/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowFunctionTypeAnnotation,
  flowFunctionTypeAnnotation,
} from '@romejs/js-ast';
import FunctionT from '../../types/FunctionT';
import MaybeT from '../../types/MaybeT';

export default function FlowFunctionTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowFunctionTypeAnnotation.assert(node);

  const bodyScope = scope.fork();
  if (node.typeParameters) {
    bodyScope.evaluate(node.typeParameters);
  }

  // build param types
  const params = [];
  let rest;
  for (const paramNode of node.params) {
    let paramType = bodyScope.evaluate(paramNode.meta.typeAnnotation);
    if (paramNode.meta.optional === true) {
      paramType = new MaybeT(scope, paramNode, paramType);
    }
    params.push(paramType);
  }
  if (node.rest !== undefined) {
    rest = bodyScope.evaluate(node.rest.meta.typeAnnotation);
  }

  // build return type
  const returns = bodyScope.evaluate(node.returnType);

  // create the function
  return new FunctionT(scope, node, {params, rest, returns, body: undefined});
}
