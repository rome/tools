/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {markup} from '@romejs/string-markup';

function isAssignment(path: Path) {
  switch (path.parentPath.node.type) {
    case 'AssignmentExpression':
    case 'AssignmentArrayPattern':
    case 'AssignmentObjectPatternProperty':
    case 'UpdateExpression':
    case 'AssignmentObjectPattern':
    case 'ForInStatement':
      return true;
  }
}

export default {
  name: 'noImportAssign',
  enter(path: Path): AnyNode {
    const {node, scope} = path;

    if (
      (node.type === 'AssignmentIdentifier' && isAssignment(path)) ||
      (node.type === 'ReferenceIdentifier' &&
        path.parentPath.node.type === 'UpdateExpression')
    ) {
      let binding = scope.getBinding(node.name);
      if (binding !== undefined && binding.kind === 'import')
        path.context.addNodeDiagnostic(node, {
          category: 'lint/noImportAssign',
          message: markup`<emphasis>${node.name}</emphasis> is read-only`,
        });
    }

    return node;
  },
};
