/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowObjectTypeIndexer} from '@romejs/js-ast';

export default function FlowObjectTypeIndexer(
  builder: Builder,
  node: FlowObjectTypeIndexer,
): Token {
  let tokens: Array<Token> = [];
  if (node.static === true) {
    tokens.push('static');
    tokens.push(space);
  }

  tokens.push(builder.tokenize(node.variance, node), '[');

  if (node.id !== undefined) {
    tokens.push(builder.tokenize(node.id, node), ':');
  }

  return concat([
    concat(tokens),
    space,
    builder.tokenize(node.key, node),
    ']',
    ':',
    space,
    builder.tokenize(node.value, node),
  ]);
}
