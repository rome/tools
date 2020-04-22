/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, newline, operator, space} from '../../tokens';
import {
  AnyNode,
  IntersectionTypeAnnotation,
  intersectionTypeAnnotation,
} from '@romejs/js-ast';

export default function IntersectionTypeAnnotation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = intersectionTypeAnnotation.assert(node);

  return [
    builder.tokenizeJoin(node.types, node, {
      newline: false,
      broken: {
        indentNewline: false,
        leading: [newline, operator('&'), space],
        separator: [newline, operator('&'), space],
      },
      unbroken: {
        separator: [space, operator('&'), space],
      },
    }),
  ];
}
