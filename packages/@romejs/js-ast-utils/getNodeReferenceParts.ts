/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';

type Parts = Array<{
  value: string;
  node: AnyNode;
}>;

export default function getNodeReferenceParts(
  node: AnyNode,
): {
  bailed: boolean;
  parts: Parts;
} {
  const parts: Parts = [];

  function add(node: AnyNode): boolean {
    if (node.type === 'Identifier' || node.type === 'ReferenceIdentifier') {
      parts.push({node, value: node.name});
      return false;
    } else if (node.type === 'StringLiteral') {
      parts.push({node, value: node.value});
      return false;
    } else if (node.type === 'MetaProperty') {
      parts.push({node, value: node.meta.name});
      parts.push({node, value: node.property.name});
      return false;
    } else if (node.type === 'MemberExpression') {
      const stop = add(node.object);
      if (stop) {
        return true;
      } else {
        return add(node.property);
      }
    } else if (node.type === 'ThisExpression') {
      return false;
    } else if (
      node.type === 'ComputedMemberProperty' &&
      node.value.type === 'StringLiteral'
    ) {
      return add(node.value);
    } else if (node.type === 'StaticMemberProperty') {
      return add(node.value);
    } else {
      return true;
    }
  }

  const bailed = add(node);

  return {bailed, parts};
}
