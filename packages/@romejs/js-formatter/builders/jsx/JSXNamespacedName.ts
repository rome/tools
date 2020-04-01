/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {JSXNamespacedName, jsxNamespacedName, AnyNode} from '@romejs/js-ast';

export default function JSXNamespacedName(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = jsxNamespacedName.assert(node);

  return [
    ...builder.tokenize(node.namespace, node),
    operator(':'),
    ...builder.tokenize(node.name, node),
  ];
}
