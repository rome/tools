/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim, concat} from '../../tokens';
import {AnyNode, regExpAlternation} from '@romejs/js-ast';

export default function RegExpAlternation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpAlternation.assert(node);

  return [
    concat(builder.tokenize(node.left, node)),
    verbatim('|'),
    concat(builder.tokenize(node.right, node)),
  ];
}
