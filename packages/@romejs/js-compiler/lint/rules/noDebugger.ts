/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from '@romejs/diagnostics';
import {Path, REDUCE_REMOVE, TransformExitResult} from '@romejs/js-compiler';

export default {
  name: 'noDebugger',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (node.type === 'DebuggerStatement') {
      return path.context.addFixableDiagnostic({
        old: node,
        fixed: REDUCE_REMOVE,
      }, descriptions.LINT.NO_DEBUGGER);
    }

    return node;
  },
};
