/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator} from '../../tokens';
import {
  BindingAssignmentPattern,
  bindingAssignmentPattern,
  AnyNode,
} from '@romejs/js-ast';

export default function BindingAssignmentPattern(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = bindingAssignmentPattern.assert(node);

  return [
    ...generator.print(node.left, node),
    space,
    operator('='),
    space,
    ...generator.print(node.right, node),
  ];
}
