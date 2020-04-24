/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator} from '../../tokens';
import {AnyNode, jsxSpreadAttribute} from '@romejs/js-ast';

export default function JSXSpreadAttribute(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = jsxSpreadAttribute.assert(node);

  return [
    operator('{'),
    operator('...'),
    concat(builder.tokenize(node.argument, node)),
    operator('}'),
  ];
}
