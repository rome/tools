/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  RegExpSubExpression,
  regExpSubExpression,
} from '@romejs/js-ast';

export default function RegExpSubExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpSubExpression.assert(node);

  for (const item of node.body) {
    generator.print(item, node);
  }
}
