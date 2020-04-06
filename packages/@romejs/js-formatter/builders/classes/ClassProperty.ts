/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, operator, concat} from '../../tokens';
import {classProperty, AnyNode} from '@romejs/js-ast';

export default function ClassProperty(builder: Builder, node: AnyNode): Tokens {
  node = classProperty.assert(node);

  if (node.value === undefined && !builder.options.typeAnnotations) {
    // A ClassProperty with no value is a type annotation
    return [];
  }

  const tokens: Tokens = [
    concat(builder.tokenize(node.meta, node)),
    concat(builder.tokenize(node.key, node)),
    concat(builder.tokenizeTypeColon(node.typeAnnotation, node)),
  ];

  if (node.value) {
    tokens.push(space);
    tokens.push(operator('='));
    tokens.push(space);
    tokens.push(concat(builder.tokenize(node.value, node)));
  }

  tokens.push(operator(';'));

  return tokens;
}
