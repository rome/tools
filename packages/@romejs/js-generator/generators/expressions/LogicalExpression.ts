/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {LogicalExpression, logicalExpression, AnyNode} from '@romejs/js-ast';
import AssignmentExpression from './AssignmentExpression';

export default function LogicalExpression(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = logicalExpression.assert(node);
  return AssignmentExpression(generator, node, parent);
}
