/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, concat} from '../../tokens';
import {jsxMemberExpression, AnyNode} from '@romejs/js-ast';

export default function JSXMemberExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = jsxMemberExpression.assert(node);

  return [
    concat(builder.tokenize(node.object, node)),
    operator('.'),
    concat(builder.tokenize(node.property, node)),
  ];
}
