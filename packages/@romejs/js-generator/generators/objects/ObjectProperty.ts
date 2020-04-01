/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
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

export default function ObjectProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = node.type === 'BindingObjectPatternProperty' || node.type ===
    'AssignmentObjectPatternProperty' ? node : objectProperty.assert(node);

  const tokens = generator.print(node.key, node);

  if ((node.value.type === 'BindingAssignmentPattern' || node.value.type ===
      'AssignmentAssignmentPattern') && isShorthand(node.key, node.value.left)) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.value.right, node.value),
    ];
  } else if (!isShorthand(node.key, node.value)) {
    return [
      ...tokens,
      operator(':'),
      space,
      ...generator.print(node.value, node),
    ];
  } else {
    return tokens;
  }
}
