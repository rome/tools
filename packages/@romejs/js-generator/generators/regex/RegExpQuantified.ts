/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, verbatim} from '../../tokens';
import {AnyNode, RegExpQuantified, regExpQuantified} from '@romejs/js-ast';

export default function RegExpQuantified(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = regExpQuantified.assert(node);

  const tokens: Tokens = generator.print(node.target, node);

  if (node.min === 0 && node.max === 1) {
    tokens.push(verbatim('?'));
  } else if (node.min === 0 && node.max === undefined) {
    tokens.push(verbatim('*'));
  } else if (node.min === 1 && node.max === undefined) {
    tokens.push(verbatim('+'));
  } else {
    tokens.push(verbatim('{'));

    tokens.push(verbatim(String(node.min)));

    if (node.min !== node.max) {
      tokens.push(verbatim(','));
      if (node.max !== undefined) {
        tokens.push(verbatim(String(node.max)));
      }
    }

    tokens.push(verbatim('}'));
  }

  if (node.lazy) {
    tokens.push(verbatim('?'));
  }

  return tokens;
}
