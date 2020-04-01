/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, flatten} from '../../tokens';
import {
  AnyNode,
  RegExpSubExpression,
  regExpSubExpression,
} from '@romejs/js-ast';

export default function RegExpSubExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpSubExpression.assert(node);
  return flatten(node.body.map((item) => builder.print(item, node)));
}
