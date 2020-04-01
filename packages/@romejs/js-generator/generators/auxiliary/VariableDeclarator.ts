/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {VariableDeclarator, variableDeclarator, AnyNode} from '@romejs/js-ast';
import {operator, space} from '@romejs/js-generator/tokens';

export default function VariableDeclarator(
  generator: Generator,
  node: AnyNode,
) {
  node = variableDeclarator.assert(node);

  if (node.init) {
    return [
      ...generator.print(node.id, node),
      space,
      operator('='),
      space,
      ...generator.print(node.init, node),
    ];
  } else {
    return generator.print(node.id, node);
  }
}
