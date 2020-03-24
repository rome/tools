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
  name: 'noVar',
  enter(path: Path): AnyNode {
    const {context, node: declaration} = path;

    if (declaration.type === 'VariableDeclaration' && declaration.kind === 'var') {
      context.addNodeDiagnostic(declaration, descriptions.LINT.NO_VAR);
    }

    return declaration;
  },
};
