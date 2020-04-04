/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {RegExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(
  builder: Builder,
  node: RegExpCharSet,
): Token {
  const tokens: Array<Token> = ['['];

  if (node.invert) {
    tokens.push('^');
  }

  return concat([
    concat(tokens),
    concat(node.body.map((item) => builder.tokenize(item, node))),
    ']',
  ]);
}
