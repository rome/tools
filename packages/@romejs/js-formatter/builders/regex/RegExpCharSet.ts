/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, verbatim} from '../../tokens';
import {AnyNode, regExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(builder: Builder, node: AnyNode): Tokens {
  node = regExpCharSet.assert(node);

  const tokens: Tokens = [verbatim('[')];

  if (node.invert) {
    tokens.push(verbatim('^'));
  }

  return [
    concat(tokens),
    concat(node.body.map((item) => concat(builder.tokenize(item, node)))),
    verbatim(']'),
  ];
}
