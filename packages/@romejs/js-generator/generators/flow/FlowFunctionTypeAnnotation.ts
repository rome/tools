/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space} from '../../tokens';
import {
  FlowFunctionTypeAnnotation,
  flowFunctionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = flowFunctionTypeAnnotation.assert(node);

  let tokens: Tokens = [
    ...generator.print(node.typeParameters, node),
    operator('('),
    generator.printCommaList(node.params, node),
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

  return [...tokens, space, ...generator.print(node.returnType, node)];
}
