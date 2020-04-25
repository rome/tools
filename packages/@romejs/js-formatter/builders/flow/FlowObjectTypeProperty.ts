/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowObjectTypeProperty} from '@romejs/js-ast';

export default function FlowObjectTypeProperty(
  builder: Builder,
  node: FlowObjectTypeProperty,
): Token {
  const tokens: Array<Token> = [];

  if (node.static === true) {
    tokens.push('static');
    tokens.push(space);
  }

  tokens.push(
    builder.tokenize(node.variance, node),
    builder.tokenize(node.key, node),
  );

  if (node.optional === true) {
    tokens.push('?');
  }

  return concat([concat(tokens), ':', space, builder.tokenize(node.value, node)]);
}
