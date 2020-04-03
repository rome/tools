/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {TemplateLiteral, templateLiteral, AnyNode} from '@romejs/js-ast';
import {Tokens, concat} from '@romejs/js-formatter/tokens';

export default function TemplateLiteral(builder: Builder, node: AnyNode): Tokens {
  node = templateLiteral.assert(node);

  const tokens: Tokens = [];

  const quasis = node.quasis;

  for (let i = 0; i < quasis.length; i++) {
    tokens.push(concat(builder.tokenize(quasis[i], node)));

    if (i + 1 < quasis.length) {
      tokens.push(concat(builder.tokenize(node.expressions[i], node)));
    }
  }

  return tokens;
}
