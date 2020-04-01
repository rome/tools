/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim} from '../../tokens';
import {AnyNode, RegExpAlternation, regExpAlternation} from '@romejs/js-ast';

export default function RegExpAlternation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpAlternation.assert(node);

  return [
    ...builder.print(node.left, node),
    verbatim('|'),
    ...builder.print(node.right, node),
  ];
}
