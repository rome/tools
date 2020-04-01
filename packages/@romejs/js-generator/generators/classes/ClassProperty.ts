/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator} from '../../tokens';
import {ClassProperty, classProperty, AnyNode} from '@romejs/js-ast';

export default function ClassProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = classProperty.assert(node);

  if (node.value === undefined && !generator.options.typeAnnotations) {
    // A ClassProperty with no value is a type annotation
    return [];
  }

  const tokens: Tokens = [
    ...generator.print(node.meta, node),
    ...generator.print(node.key, node),
    ...generator.printTypeColon(node.typeAnnotation, node),
  ];

  if (node.value) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.value, node),
      operator(';'),
    ];
  } else {
    return [...tokens, operator(';')];
  }
}
