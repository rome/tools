/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AssignmentObjectPattern,
  assignmentObjectPattern,
  AnyNode,
} from '@romejs/js-ast';
import ObjectExpression from '../objects/ObjectExpression';

export default function AssignmentObjectPattern(
  generator: Generator,
  node: AnyNode,
) {
  node = assignmentObjectPattern.assert(node);

  ObjectExpression(generator, node);
}
