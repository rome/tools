/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {binaryExpression, AnyNode} from '@romejs/js-ast';
import AssignmentExpression from './AssignmentExpression';

export default function BinaryExpression(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = binaryExpression.assert(node);
  return AssignmentExpression(builder, node, parent);
}
