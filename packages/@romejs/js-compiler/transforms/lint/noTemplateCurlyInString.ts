/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {markup} from '@romejs/string-markup';

export default {
  name: 'noTemplateCurlyInString',
  enter(path: Path) {
    const {node, context} = path;

    if (node.type === 'StringLiteral') {
      const regex = /\$\{[^}]+\}/u;

      if (regex.test(node.value)) {
        context.addNodeDiagnostic(node, {
          category: 'lint/noTemplateCurlyInString',
          message: markup`Unexpected template string expression.`,
        });
      }
    }

    return node;
  },
};
