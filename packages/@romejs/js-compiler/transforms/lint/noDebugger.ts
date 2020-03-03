/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {REDUCE_REMOVE} from '@romejs/js-compiler';

export default {
  name: 'noDebugger',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type === 'DebuggerStatement') {
      path.context.addNodeDiagnostic(node, {
        fixable: true,
        category: 'lint/noDebugger',
        message: "Unexpected 'debugger' statement",
      });
    }

    return REDUCE_REMOVE;
  },
};
