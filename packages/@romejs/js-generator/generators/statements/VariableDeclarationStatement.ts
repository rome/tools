/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  VariableDeclarationStatement,
  variableDeclarationStatement,
  AnyNode,
} from '@romejs/js-ast';

export default function VariableDeclarationStatement(
  generator: Generator,
  node: AnyNode,
) {
  node = variableDeclarationStatement.assert(node);

  if (node.declare === true && !generator.options.typeAnnotations) {
    return;
  }

  if (node.declare) {
    generator.word('declare');
  }

  generator.print(node.declaration, node);
  generator.semicolon();
}
