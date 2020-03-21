/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'noUnsafeFinally',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'TryStatement') {
      const {finalizer} = node;

      if (finalizer && finalizer.type === 'BlockStatement') {
        for (const statement of finalizer.body) {
          if (statement.type === 'ThrowStatement' || statement.type ===
          'ContinueStatement' || statement.type === 'BreakStatement' ||
          statement.type === 'ReturnStatement') {
            context.addNodeDiagnostic(statement, {
              category: 'lint/noUnsafeFinally',
              message: `Unsafe usage of ${statement.type}.`,
            });
          }
        }
      }
    }

    return node;
  },
};
