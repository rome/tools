/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space} from '../../tokens';
import {DoExpression, doExpression, AnyNode} from '@romejs/js-ast';

export default function DoExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = doExpression.assert(node);

  return [word('do'), space, ...generator.print(node.body, node)];
}
