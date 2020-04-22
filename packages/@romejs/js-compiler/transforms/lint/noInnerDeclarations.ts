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
  name: 'noInnerDeclarations',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'FunctionDeclaration') {
      const parentScopeKind = path.parentPath.parentPath.scope.kind;
      if (parentScopeKind !== 'block' && parentScopeKind !== 'loop') {
        return node;
      }
      const upperFunction = path.findAncestry((p) => p.scope.kind === 'function');
      const body = upperFunction ? 'function body' : 'program';

      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.NO_INNER_DECLARATIONS(body),
      );
    }

    return node;
  },
};
