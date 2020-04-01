/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space} from '../../tokens';
import {
  ObjectProperty,
  objectProperty,
  AnyNode,
  AnyObjectPropertyKey,
} from '@romejs/js-ast';

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

  const tokens = builder.print(node.key, node);

  if ((node.value.type === 'BindingAssignmentPattern' || node.value.type ===
      'AssignmentAssignmentPattern') && isShorthand(node.key, node.value.left)) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.print(node.value.right, node.value),
    ];
  } else if (!isShorthand(node.key, node.value)) {
    return [
      ...tokens,
      operator(':'),
      space,
      ...builder.print(node.value, node),
    ];
  } else {
    return tokens;
  }
}
