/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'noExtraElseInRegEx',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'RegExpSubExpression') {
      if (node.body.length === 0) {
        context.addNodeDiagnostic(node, {
          category: 'lint/noExtraElseInRegEx',
          message: `Extra else in regular expressions is not allowed.`,
        });
      }
    }

    return node;
  },
};
