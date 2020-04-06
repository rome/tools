/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, referenceIdentifier, arrayExpression} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'sparseArray',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'ArrayExpression' && node.elements.includes(undefined)) {
      const {suppressed} = path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.SPARSE_ARRAY,
      );

      if (!suppressed) {
        return arrayExpression.quick(node.elements.map((elem) => elem ===
          undefined ? referenceIdentifier.create({name: 'undefined'}) : elem));
      }
    }

    return node;
  },
};
