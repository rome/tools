/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, newline, operator} from '../../tokens';
import {
  IntersectionTypeAnnotation,
  intersectionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function IntersectionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = intersectionTypeAnnotation.assert(node);

  return [
    generator.printJoin(node.types, node, {
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
