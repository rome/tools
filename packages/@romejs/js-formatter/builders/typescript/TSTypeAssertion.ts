/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeAssertion, tsTypeAssertion, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space} from '../../tokens';

export default function TSTypeAssertion(builder: Builder, node: AnyNode): Tokens {
  node = tsTypeAssertion.assert(node);

  if (builder.options.typeAnnotations) {
    return [
      operator('<'),
      ...builder.tokenize(node.typeAnnotation, node),
      operator('>'),
      space,
      ...builder.tokenize(node.expression, node),
    ];
  } else {
    return builder.tokenize(node.expression, node);
  }
}
