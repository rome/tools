/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  AnyNode,
  TSAssignmentNonNullExpression,
  tsAssignmentNonNullExpression,
} from '@romejs/js-ast';

export default function TSAssignmentNonNullExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsAssignmentNonNullExpression.assert(node);

  return [...builder.tokenize(node.expression, node), operator('!')];
}
