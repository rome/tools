/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {JSXSpreadAttribute, jsxSpreadAttribute, AnyNode} from '@romejs/js-ast';

export default function JSXSpreadAttribute(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = jsxSpreadAttribute.assert(node);

  return [
    operator('{'),
    operator('...'),
    ...generator.print(node.argument, node),
    operator('}'),
  ];
}
