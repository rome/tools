/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, concat} from '../../tokens';
import {jsxExpressionContainer, AnyNode} from '@romejs/js-ast';

export default function JSXExpressionContainer(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = jsxExpressionContainer.assert(node);

  return [
    operator('{'),
    concat(builder.tokenize(node.expression, node)),
    operator('}'),
  ];
}
