/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space} from '../../tokens';
import {
  AnyNode,
  TSAssignmentTypeAssertion,
  tsAssignmentTypeAssertion,
} from '@romejs/js-ast';

export default function TSAssignmentTypeAssertion(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsAssignmentTypeAssertion.assert(node);

  return [
    operator('<'),
    ...builder.tokenize(node.typeAnnotation, node),
    operator('>'),
    space,
    ...builder.tokenize(node.expression, node),
  ];
}
