/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {JSXNamespacedName, jsxNamespacedName, AnyNode} from '@romejs/js-ast';

export default function JSXNamespacedName(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = jsxNamespacedName.assert(node);

  return [
    ...generator.print(node.namespace, node),
    operator(':'),
    ...generator.print(node.name, node),
  ];
}
