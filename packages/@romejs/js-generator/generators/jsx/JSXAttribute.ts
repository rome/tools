/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {JSXAttribute, jsxAttribute, AnyNode} from '@romejs/js-ast';

export default function JSXAttribute(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = jsxAttribute.assert(node);

  const tokens: Tokens = generator.print(node.name, node);

  if (node.value) {
    return [...tokens, operator('='), ...generator.print(node.value, node)];
  } else {
    return tokens;
  }
}
