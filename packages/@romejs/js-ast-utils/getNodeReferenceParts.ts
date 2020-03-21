/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';

export default function getNodeReferenceParts(
  node: AnyNode,
): [boolean, Array<string>] {
  const parts: Array<string> = [];

  function add(node: AnyNode): boolean {
    if (node.type === 'Identifier' || node.type === 'ReferenceIdentifier') {
      parts.push(node.name);
      return false;
    } else if (node.type === 'StringLiteral') {
      parts.push(node.value);
      return false;
    } else if (node.type === 'MetaProperty') {
      parts.push(node.meta.name);
      parts.push(node.property.name);
      return false;
    } else if (node.type === 'MemberExpression') {
      const stop = add(node.object);
      if (stop) {
        return true;
      } else {
        return add(node.property);
      }
    } else if (node.type === 'ComputedMemberProperty' && node.value.type ===
    'StringLiteral') {
      return add(node.value);
    } else if (node.type === 'StaticMemberProperty') {
      return add(node.value);
    } else {
      return true;
    }
  }

  const bailed = add(node);

  return [bailed, parts];
}
