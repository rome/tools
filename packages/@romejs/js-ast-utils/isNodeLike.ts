/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export default function isNodeLike(node: unknown): boolean {
  if (node == null) {
    return false;
  } else {
    return (
      // @ts-ignore
      typeof node === 'object' && node !== null && typeof node.type === 'string'
    );
  }
}
