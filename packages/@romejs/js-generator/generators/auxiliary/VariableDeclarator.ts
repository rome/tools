/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {VariableDeclarator, variableDeclarator, AnyNode} from '@romejs/js-ast';

export default function VariableDeclarator(
  generator: Generator,
  node: AnyNode,
) {
  node = variableDeclarator.assert(node);

  const {id} = node;
  generator.print(id, node);
  if (id.meta !== undefined) {
    generator.print(id.meta.typeAnnotation, id.meta);
  }

  if (node.init) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.init, node);
  }
}
