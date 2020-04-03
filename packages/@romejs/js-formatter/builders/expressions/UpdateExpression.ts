/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, concat} from '../../tokens';
import {updateExpression, AnyNode} from '@romejs/js-ast';

export default function UpdateExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = updateExpression.assert(node);

  if (node.prefix === true) {
    return [
      operator(node.operator),
      concat(builder.tokenize(node.argument, node)),
    ];
  } else {
    return [
      concat(builder.tokenize(node.argument, node)),
      operator(node.operator),
    ];
  }
}
