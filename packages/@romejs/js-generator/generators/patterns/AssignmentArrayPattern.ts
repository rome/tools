/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AssignmentArrayPattern,
  assignmentArrayPattern,
  AnyNode,
} from '@romejs/js-ast';
import ArrayExpression from '../expressions/ArrayExpression';

export default function AssignmentArrayPattern(
  generator: Generator,
  node: AnyNode,
) {
  node = assignmentArrayPattern.assert(node);

  ArrayExpression(generator, node);
}
