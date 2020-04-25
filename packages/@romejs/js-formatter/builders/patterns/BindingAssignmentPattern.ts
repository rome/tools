/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space} from '../../tokens';
import {
  AnyNode,
  BindingAssignmentPattern,
  bindingAssignmentPattern,
} from '@romejs/js-ast';

export default function BindingAssignmentPattern(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'AssignmentAssignmentPattern'
    ? node
    : bindingAssignmentPattern.assert(node);

  return [
    concat(builder.tokenize(node.left, node)),
    space,
    operator('='),
    space,
    concat(builder.tokenize(node.right, node)),
  ];
}
