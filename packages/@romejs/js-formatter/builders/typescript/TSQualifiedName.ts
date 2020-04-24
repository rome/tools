/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSQualifiedName, tsQualifiedName} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSQualifiedName(builder: Builder, node: AnyNode): Tokens {
  node = tsQualifiedName.assert(node);

  return [
    ...builder.tokenize(node.left, node),
    operator('.'),
    ...builder.tokenize(node.right, node),
  ];
}
