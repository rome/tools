/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, ConditionalExpression, IfStatement} from '@romejs/js-ast';

export default function isConditional(
  node: undefined | AnyNode,
): node is ConditionalExpression | IfStatement {
  if (node === undefined) {
    return false;
  }

  switch (node.type) {
    case 'ConditionalExpression':
    case 'IfStatement':
      return true;

    default:
      return false;
  }
}
