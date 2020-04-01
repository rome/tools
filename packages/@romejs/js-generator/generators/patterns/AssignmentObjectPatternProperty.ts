/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  AssignmentObjectPatternProperty,
  assignmentObjectPatternProperty,
  AnyNode,
} from '@romejs/js-ast';
import ObjectProperty from '../objects/ObjectProperty';

export default function AssignmentObjectPatternProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = assignmentObjectPatternProperty.assert(node);
  return ObjectProperty(generator, node);
}
