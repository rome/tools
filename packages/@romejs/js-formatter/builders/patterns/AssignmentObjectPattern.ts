/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  AssignmentObjectPattern,
  assignmentObjectPattern,
} from '@romejs/js-ast';
import ObjectExpression from '../objects/ObjectExpression';

export default function AssignmentObjectPattern(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = assignmentObjectPattern.assert(node);
  return ObjectExpression(builder, node);
}
