/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'invalidQuantifierRange',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (
      node.type === 'RegExpQuantified' &&
      node.max !== undefined &&
      node.min > node.max
    ) {
      context.addNodeDiagnostic(node, {
        category: 'lint/invalidQuantifierRange',
        message: 'Quantifier minimum is greater than maximum',
      });
    }

    return node;
  },
};
