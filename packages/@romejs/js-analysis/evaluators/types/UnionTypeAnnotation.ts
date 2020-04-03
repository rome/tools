/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  UnionTypeAnnotation,
  unionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';
import UnionT from '../../types/UnionT';

export default function UnionTypeAnnotation(node: AnyNode, scope: Scope) {
  node = unionTypeAnnotation.assert(node);

  return new UnionT(scope, node, node.types.map((type) => {
    return scope.evaluate(type);
  }));
}
