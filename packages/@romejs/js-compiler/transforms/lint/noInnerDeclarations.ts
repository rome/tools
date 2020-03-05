/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noInnerDeclarations',
  enter(path: Path): AnyNode {
    const {context, node: declaration} = path;

    if (
      declaration.type === 'FunctionDeclaration' &&
      path.parent.type !== 'Program'
    ) {
      if (
        [
          'Program',
          'FunctionDeclaration',
          'FunctionExpression',
          'ArrowFunctionExpression',
        ].indexOf(path.parentPath.parentPath.node.type) < 0
      ) {
        context.addNodeDiagnostic(declaration, {
          category: 'lint/noInnerDeclarations',
          message: 'Function declarations in nested blocks are not permitted.',
        });
      }
    }

    return declaration;
  },
};
