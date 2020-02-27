/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FunctionExpression, functionExpression, AnyNode} from '@romejs/js-ast';

export default function FunctionExpression(
  generator: Generator,
  node: AnyNode,
) {
  node =
    node.type === 'FunctionDeclaration'
      ? node
      : functionExpression.assert(node);

  if (node.head.async === true) {
    generator.word('async');
    generator.space();
  }

  generator.word('function');

  if (node.head.generator === true) {
    generator.token('*');
  }

  if (node.id) {
    generator.space();
    generator.print(node.id, node);
  }

  generator.print(node.head, node);
  generator.space();
  generator.print(node.body, node);
}
