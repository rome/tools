/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space} from '../../tokens';
import {
  FlowDeclaredPredicate,
  flowDeclaredPredicate,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclaredPredicate(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowDeclaredPredicate.assert(node);

  return [operator('%checks'), space, ...generator.print(node.value, node)];
}
