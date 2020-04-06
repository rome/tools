/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space} from '../../tokens';
import {
  FlowFunctionTypeAnnotation,
  flowFunctionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeAnnotation(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = flowFunctionTypeAnnotation.assert(node);

  let tokens: Tokens = [
    ...builder.tokenize(node.typeParameters, node),
    operator('('),
    builder.tokenizeCommaList(node.params, node),
    operator(')'),
  ];

  // this node type is overloaded, not sure why but it makes it EXTREMELY annoying
  if (parent.type === 'FlowObjectTypeCallProperty' || parent.type ===
      'FlowDeclareFunction') {
    tokens.push(operator(':'));
  } else {
    tokens.push(space);
    tokens.push(operator('=>'));
  }

  return [...tokens, space, ...builder.tokenize(node.returnType, node)];
}
