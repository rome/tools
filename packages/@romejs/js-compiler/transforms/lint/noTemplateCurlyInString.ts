/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noTemplateCurlyInString',
  enter(path: Path) {
    const {node, context} = path;

    if (node.type === 'StringLiteral') {
      const regex = /\$\{[^}]+\}/u;

      if (regex.test(node.value)) {
        context.addNodeDiagnostic(node, {
          description: descriptions.LINT.NO_TEMPLATE_CURLY_IN_STRING,
        });
      }
    }

    return node;
  },
};
