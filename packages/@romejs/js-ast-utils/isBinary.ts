/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, BinaryExpression, LogicalExpression} from '@romejs/js-ast';

export default function isBinary(
  node: undefined | AnyNode,
): node is BinaryExpression | LogicalExpression {
  if (node === undefined) {
    return false;
  }

  switch (node.type) {
    case 'BinaryExpression':
    case 'LogicalExpression':
      return true;

    default:
      return false;
  }
}
