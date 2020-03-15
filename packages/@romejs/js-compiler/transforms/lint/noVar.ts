/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noVar',
  enter(path: Path): AnyNode {
    const {context, node: declaration} = path;

    if (
      declaration.type === 'VariableDeclaration' &&
      declaration.kind === 'var'
    ) {
      context.addNodeDiagnostic(declaration, {
        category: 'lint/noVar',
        message:
          'Variable declarations using `var` are disallowed, use `let` or `const` instead.',
      });
    }

    return declaration;
  },
};
