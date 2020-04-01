/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {MemberExpression, memberExpression, AnyNode} from '@romejs/js-ast';

export default function MemberExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = memberExpression.assert(node);

  return [
    ...builder.print(node.object, node),
    ...builder.print(node.property, node),
  ];
}
