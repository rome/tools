/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, concat} from '../../tokens';
import {objectProperty, AnyNode, AnyObjectPropertyKey} from '@romejs/js-ast';

function isShorthand(key: AnyObjectPropertyKey, value: AnyNode): boolean {
  return key.type === 'StaticPropertyKey' && key.value.type === 'Identifier' &&
      (value.type === 'ReferenceIdentifier' || value.type ===
          'BindingIdentifier' ||
        value.type === 'AssignmentIdentifier') &&
    value.name === key.value.name;
}

export default function ObjectProperty(builder: Builder, node: AnyNode): Tokens {
  node = node.type === 'BindingObjectPatternProperty' || node.type ===
    'AssignmentObjectPatternProperty' ? node : objectProperty.assert(node);

  const tokens = builder.tokenize(node.key, node);

  if ((node.value.type === 'BindingAssignmentPattern' || node.value.type ===
      'AssignmentAssignmentPattern') && isShorthand(node.key, node.value.left)) {
    return [
      concat(tokens),
      space,
      operator('='),
      space,
      concat(builder.tokenize(node.value.right, node.value)),
    ];
  } else if (!isShorthand(node.key, node.value)) {
    return [
      concat(tokens),
      operator(':'),
      space,
      concat(builder.tokenize(node.value, node)),
    ];
  } else {
    return tokens;
  }
}
