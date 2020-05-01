/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noEmptyCharacterClass',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;
    if (node.type === 'RegExpSubExpression') {
      let isQuantifiedMinZero = el => el.type === 'RegExpQuantified' && el.min == 0;
      if (node.body.every(isQuantifiedMinZero)) {
        context.addNodeDiagnostic(node, descriptions.LINT.EMPTY_MATCHES);
      }
    }
    return node;
  },
};
