/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'reactInJsxScope',
  enter(path: Path): AnyNode {
    const {node, scope, context} = path;

    if (node.type === 'JSXElement') {
      const reactIsInScope = scope.getBinding('React') !== undefined;

      if (!reactIsInScope) {
        context.addNodeDiagnostic(node, descriptions.LINT.REACT_IN_JSX_SCOPE);
      }
    }

    return node;
  },
};
