/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {UpdateExpression, updateExpression, AnyNode} from '@romejs/js-ast';

export default function UpdateExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = updateExpression.assert(node);

  if (node.prefix === true) {
    return [operator(node.operator), ...generator.print(node.argument, node)];
  } else {
    return [...generator.print(node.argument, node), operator(node.operator)];
  }
}
