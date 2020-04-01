/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, flatten} from '../../tokens';
import {
  AnyNode,
  RegExpSubExpression,
  regExpSubExpression,
} from '@romejs/js-ast';

export default function RegExpSubExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = regExpSubExpression.assert(node);
  return flatten(node.body.map(item => generator.print(item, node)));
}
