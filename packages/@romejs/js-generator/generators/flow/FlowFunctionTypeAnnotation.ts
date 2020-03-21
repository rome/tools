/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowFunctionTypeAnnotation,
  flowFunctionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowFunctionTypeAnnotation.assert(node);

  generator.print(node.typeParameters, node);
  generator.token('(');
  generator.printCommaList(node.params, node);
  generator.token(')');

  // this node type is overloaded, not sure why but it makes it EXTREMELY annoying
  if (parent.type === 'FlowObjectTypeCallProperty' || parent.type ===
  'FlowDeclareFunction') {
    generator.token(':');
  } else {
    generator.space();
    generator.token('=>');
  }

  generator.space();
  generator.print(node.returnType, node);
}
