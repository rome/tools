/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {AnyNode, LogicalExpression} from '@romejs/js-ast';
import BinaryExpression from './BinaryExpression';

export default function LogicalExpression(
  builder: Builder,
  node: LogicalExpression,
  parent: AnyNode,
): Token {
  return BinaryExpression(builder, node, parent);
}
