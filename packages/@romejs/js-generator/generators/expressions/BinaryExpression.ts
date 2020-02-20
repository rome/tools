/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {binaryExpression, AnyNode} from '@romejs/js-ast';
import AssignmentExpression from './AssignmentExpression';

export default function BinaryExpression(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = binaryExpression.assert(node);
  AssignmentExpression(generator, node, parent);
}
