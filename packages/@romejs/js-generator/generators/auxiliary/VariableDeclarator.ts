/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {VariableDeclarator, variableDeclarator, AnyNode} from '@romejs/js-ast';

export default function VariableDeclarator(generator: Generator, node: AnyNode) {
  node = variableDeclarator.assert(node);

  generator.print(node.id, node);

  if (node.init) {
    generator.space();
    generator.token('=');
    generator.space();

    generator.multiline(node, (multiline, node) => {
      generator.print(node.init, node);
    }, {indent: true});
  }
}
