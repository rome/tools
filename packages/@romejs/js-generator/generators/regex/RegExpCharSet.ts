/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, verbatim, flatten} from '../../tokens';
import {AnyNode, RegExpCharSet, regExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = regExpCharSet.assert(node);

  let tokens: Tokens = [verbatim('[')];

  if (node.invert) {
    tokens.push(verbatim('^'));
  }

  return [
    ...tokens,
    ...flatten(node.body.map((item) => generator.print(item, node))),
    verbatim(']'),
  ];
}
