/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {
  ExpressionStatement,
  expressionStatement,
  AnyNode,
} from '@romejs/js-ast';

export default function ExpressionStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = expressionStatement.assert(node);

  return [...generator.print(node.expression, node), operator(';')];
}
