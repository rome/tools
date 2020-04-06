/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AssignmentObjectPatternProperty,
  assignmentObjectPatternProperty,
  AnyNode,
} from '@romejs/js-ast';
import ObjectProperty from '../objects/ObjectProperty';

export default function AssignmentObjectPatternProperty(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = assignmentObjectPatternProperty.assert(node);
  return ObjectProperty(builder, node);
}
