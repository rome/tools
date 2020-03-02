/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noDebugger',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'DebuggerStatement') {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/noDebugger',
        message: "Unexpected 'debugger' statement",
      });
    }

    return node;
  },
};
