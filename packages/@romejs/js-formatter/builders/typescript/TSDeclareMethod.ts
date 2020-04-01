/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSDeclareMethod, tsDeclareMethod, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens} from '../../tokens';
import {printMethod} from '../utils';

export default function TSDeclareMethod(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsDeclareMethod.assert(node);

  return [
    ...builder.tokenize(node.meta, node),
    ...builder.tokenize(node.key, node),
    ...printMethod(builder, node),
  ];
}
