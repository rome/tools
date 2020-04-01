/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {
  JSXMemberExpression,
  jsxMemberExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function JSXMemberExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = jsxMemberExpression.assert(node);

  return [
    ...generator.print(node.object, node),
    operator('.'),
    ...generator.print(node.property, node),
  ];
}
