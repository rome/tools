/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExpressionStatement,
  expressionStatement,
  AnyNode,
} from '@romejs/js-ast';

export default function ExpressionStatement(
  generator: Generator,
  node: AnyNode,
) {
  node = expressionStatement.assert(node);

  generator.print(node.expression, node);
  generator.semicolon();
}
