/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, blockStatement} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'caseSingleStatement',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'SwitchCase') {
      if (node.consequent.length > 1) {
        const {suppressed} = context.addNodeDiagnostic(
          node,
          descriptions.LINT.CASE_SINGLE_STATEMENT,
        );

        if (!suppressed) {
          return {
            ...node,
            consequent: [blockStatement.quick(node.consequent)],
          };
        }
      }
    }

    return node;
  },
};
