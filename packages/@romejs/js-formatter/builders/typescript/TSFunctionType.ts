/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSFunctionType, tsFunctionType, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space, linkedGroups} from '../../tokens';

export default function TSFunctionType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsFunctionType.assert(node);

  return [
    linkedGroups([
      ...builder.tokenize(node.meta, node),
      space,
      operator('=>'),
      space,
      ...builder.tokenize(node.typeAnnotation, node),
    ]),
  ];
}
