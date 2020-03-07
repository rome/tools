/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, REDUCE_REMOVE, TransformExitResult} from '@romejs/js-compiler';

export default {
  name: 'noEmptyCharacterClass',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;

    if (node.type === 'RegExpCharSet') {
      if (node.body.length === 0) {
        context.addNodeDiagnostic(node, {
          category: 'lint/noEmptyCharacterClass',
          message:
            'Empty character classes in regular expressions are not allowed',
        });
        return REDUCE_REMOVE;
      }
    }

    return node;
  },
};
