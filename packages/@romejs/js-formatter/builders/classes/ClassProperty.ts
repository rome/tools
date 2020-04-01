/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, operator} from '../../tokens';
import {ClassProperty, classProperty, AnyNode} from '@romejs/js-ast';

export default function ClassProperty(builder: Builder, node: AnyNode): Tokens {
  node = classProperty.assert(node);

  if (node.value === undefined && !builder.options.typeAnnotations) {
    // A ClassProperty with no value is a type annotation
    return [];
  }

  const tokens: Tokens = [
    ...builder.print(node.meta, node),
    ...builder.print(node.key, node),
    ...builder.printTypeColon(node.typeAnnotation, node),
  ];

  if (node.value) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.print(node.value, node),
      operator(';'),
    ];
  } else {
    return [...tokens, operator(';')];
  }
}
