/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim} from '../../tokens';
import {AnyNode, RegExpCharSetRange, regExpCharSetRange} from '@romejs/js-ast';

export default function RegExpCharSetRange(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = regExpCharSetRange.assert(node);

  return [
    ...builder.print(node.start, node),
    verbatim('-'),
    ...builder.print(node.end, node),
  ];
}
