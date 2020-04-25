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
  name: 'noCatchAssign',
  enter(path: Path): AnyNode {
    const {node, context, scope} = path;

    if (node.type === 'AssignmentIdentifier') {
      const binding = scope.getBinding(node.name);

      if (binding !== undefined && binding.kind === 'catch') {
        context.addNodeDiagnostic(node, descriptions.LINT.NO_CATCH_ASSIGN);
      }
    }

    return node;
  },
};
