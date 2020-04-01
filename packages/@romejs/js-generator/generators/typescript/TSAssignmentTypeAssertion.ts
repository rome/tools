/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space} from '../../tokens';
import {
  TSAssignmentTypeAssertion,
  tsAssignmentTypeAssertion,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentTypeAssertion(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsAssignmentTypeAssertion.assert(node);

  return [
    operator('<'),
    ...generator.print(node.typeAnnotation, node),
    operator('>'),
    space,
    ...generator.print(node.expression, node),
  ];
}
