/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, arrowFunctionExpression} from '@romejs/js-ast';

export default function ArrowFunctionExpression(builder: Builder, node: AnyNode) {
  node = arrowFunctionExpression.assert(node);

  const tokens: Tokens = [];

  if (node.head.async === true) {
    tokens.push(word('async'));
    tokens.push(space);
  }

  return [
    concat(tokens),
    concat(builder.tokenize(node.head, node)),
    space,
    operator('=>'),
    space,
    concat(builder.tokenize(node.body, node)),
  ];
}
