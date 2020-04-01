/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  FlowTypeCastExpression,
  flowTypeCastExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTypeCastExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowTypeCastExpression.assert(node);

  if (builder.options.typeAnnotations) {
    return [
      operator('('),
      ...builder.print(node.expression, node),
      ...builder.printTypeColon(node.typeAnnotation, node),
      operator(')'),
    ];
  } else {
    return builder.print(node.expression, node);
  }
}
