/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AssignmentArrayPattern,
  assignmentArrayPattern,
  AnyNode,
} from '@romejs/js-ast';
import ArrayExpression from '../expressions/ArrayExpression';

export default function AssignmentArrayPattern(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = assignmentArrayPattern.assert(node);
  return ArrayExpression(builder, node);
}
