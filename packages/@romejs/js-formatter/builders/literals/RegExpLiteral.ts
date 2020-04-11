/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat} from '../../tokens';
import {regExpLiteral, AnyNode} from '@romejs/js-ast';
import {operator} from '@romejs/js-formatter/tokens';

export default function RegExpLiteral(builder: Builder, node: AnyNode): Tokens {
  node = regExpLiteral.assert(node);

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

  return [
    operator('/'),
    concat(builder.tokenize(node.expression, node)),
    operator(`/${flags.join('')}`),
  ];
}
