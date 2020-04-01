/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSTypeParameterInstantiation,
  tsTypeParameterInstantiation,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSTypeParameterInstantiation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsTypeParameterInstantiation.assert(node);

  return [
    operator('<'),
    builder.printCommaList(node.params, node),
    operator('>'),
  ];
}
