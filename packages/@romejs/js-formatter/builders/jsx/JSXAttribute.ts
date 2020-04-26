/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {JSXAttribute} from '@romejs/js-ast';

export default function JSXAttribute(
  builder: Builder,
  node: JSXAttribute,
): Token {
  const tokens: Array<Token> = [builder.tokenize(node.name, node)];

  if (node.value) {
    return concat([concat(tokens), '=', builder.tokenize(node.value, node)]);
  } else {
    return concat(tokens);
  }
}
