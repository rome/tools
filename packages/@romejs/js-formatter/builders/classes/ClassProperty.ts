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
    ...builder.tokenize(node.meta, node),
    ...builder.tokenize(node.key, node),
    ...builder.tokenizeTypeColon(node.typeAnnotation, node),
  ];

  if (node.value) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.tokenize(node.value, node),
      operator(';'),
    ];
  } else {
    return [...tokens, operator(';')];
  }
}
