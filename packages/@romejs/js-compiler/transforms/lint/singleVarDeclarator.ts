/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
  AnyNode,
  VariableDeclarationStatement,
  variableDeclarationStatement,
  variableDeclaration,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'singleVarDeclarator',
  enter(path: Path): AnyNode | Array<VariableDeclarationStatement> {
    const {node} = path;

    if (node.type === 'VariableDeclarationStatement' &&
      node.declaration.declarations.length > 1) {
      const {suppressed} = path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.SINGLE_VAR_DECLARATOR,
      );

      if (!suppressed) {
        const nodes: Array<VariableDeclarationStatement> = [];
        const {kind} = node.declaration;

        for (const declarator of node.declaration.declarations) {
          nodes.push(variableDeclarationStatement.quick(
            variableDeclaration.create({
              kind,
              declarations: [declarator],
            }),
          ));
        }

        return nodes;
      }
    }

    return node;
  },
};
