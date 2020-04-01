/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim, flatten} from '../../tokens';
import {AnyNode, RegExpCharSet, regExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(builder: Builder, node: AnyNode): Tokens {
  node = regExpCharSet.assert(node);

  let tokens: Tokens = [verbatim('[')];

  if (node.invert) {
    tokens.push(verbatim('^'));
  }

  return [
    ...tokens,
    ...flatten(node.body.map((item) => builder.tokenize(item, node))),
    verbatim(']'),
  ];
}
