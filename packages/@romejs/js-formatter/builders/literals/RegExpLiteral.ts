/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {RegExpLiteral} from '@romejs/js-ast';

export default function RegExpLiteral(
  builder: Builder,
  node: RegExpLiteral,
): Token {
  const flags: Array<string> = [];

  if (node.global === true) {
    flags.push('g');
  }

  if (node.multiline === true) {
    flags.push('m');
  }

  if (node.sticky === true) {
    flags.push('y');
  }

  if (node.insensitive === true) {
    flags.push('i');
  }

  if (node.noDotNewline === true) {
    flags.push('s');
  }

  if (node.unicode === true) {
    flags.push('u');
  }

  return concat([
    '/',
    builder.tokenize(node.expression, node),
    '/',
    flags.join(''),
  ]);
}
