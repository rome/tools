/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, BindingIdentifier} from '@romejs/js-ast';
import {bindingKeys} from '@romejs/js-ast/utils';

export default function getBindingIdentifiers(
  node: AnyNode | Array<AnyNode>,
): Array<BindingIdentifier> {
  const ids: Array<BindingIdentifier> = [];
  let queue: Array<undefined | AnyNode> = Array.isArray(node)
    ? [...node]
    : [node];

  while (queue.length) {
    const node = queue.pop();
    if (node === undefined) {
      continue;
    }

    if (node.type === 'BindingIdentifier') {
      ids.push(node);
      continue;
    }

    const keys: undefined | Array<string> = bindingKeys.get(node.type);
    if (keys === undefined) {
      continue;
    }

    for (const key of keys) {
      // rome-suppress lint/noExplicitAny
      const val = (node as any)[key];
      if (val === undefined) {
        continue;
      } else if (Array.isArray(val)) {
        queue = queue.concat(val);
      } else {
        queue.push(val);
      }
    }
  }

  return ids;
}
