/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ObjectProperty,
  objectProperty,
  AnyNode,
  AnyObjectPropertyKey,
} from '@romejs/js-ast';

function isShorthand(key: AnyObjectPropertyKey, value: AnyNode): boolean {
  return (
    key.type === 'StaticPropertyKey' &&
    key.value.type === 'Identifier' &&
    (value.type === 'ReferenceIdentifier' ||
      value.type === 'BindingIdentifier' ||
      value.type === 'AssignmentIdentifier') &&
    value.name === key.value.name
  );
}

export default function ObjectProperty(generator: Generator, node: AnyNode) {
  node =
    node.type === 'BindingObjectPatternProperty' ||
    node.type === 'AssignmentObjectPatternProperty'
      ? node
      : objectProperty.assert(node);

  generator.print(node.key, node);

  if (
    (node.value.type === 'BindingAssignmentPattern' ||
      node.value.type === 'AssignmentAssignmentPattern') &&
    isShorthand(node.key, node.value.left)
  ) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.value.right, node.value);
  } else if (!isShorthand(node.key, node.value)) {
    generator.token(':');
    generator.space();
    generator.print(node.value, node);
  }
}
