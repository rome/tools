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
  AssignmentAssignmentPattern,
  assignmentAssignmentPattern,
} from '@romejs/js-ast';
import BindingAssignmentPattern from './BindingAssignmentPattern';

export default function AssignmentAssignmentPattern(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = assignmentAssignmentPattern.assert(node);
  return BindingAssignmentPattern(builder, node);
}
