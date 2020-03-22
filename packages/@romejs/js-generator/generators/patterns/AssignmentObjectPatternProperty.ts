/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AssignmentObjectPatternProperty,
  assignmentObjectPatternProperty,
  AnyNode,
} from '@romejs/js-ast';
import ObjectProperty from '../objects/ObjectProperty';

export default function AssignmentObjectPatternProperty(
  generator: Generator,
  node: AnyNode,
) {
  node = assignmentObjectPatternProperty.assert(node);
  ObjectProperty(generator, node);
}
