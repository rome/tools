/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'jsxNoCommentText',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'JSXText') {
      if (/^\s*\/(\/|\*)/m.test(node.value)) {
        path.context.addNodeDiagnostic(
          node,
          descriptions.LINT.REACT_JSX_NO_COMMENT_TEXT,
        );
      }
    }

    return node;
  },
};
