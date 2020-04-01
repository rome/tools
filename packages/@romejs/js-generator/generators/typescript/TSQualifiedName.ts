/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSQualifiedName, tsQualifiedName, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator} from '../../tokens';

export default function TSQualifiedName(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsQualifiedName.assert(node);

  return [
    ...generator.print(node.left, node),
    operator('.'),
    ...generator.print(node.right, node),
  ];
}
