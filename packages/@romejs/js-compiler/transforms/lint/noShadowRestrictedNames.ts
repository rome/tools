/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {
  VariableDeclaration,
  FunctionDeclaration,
  CatchClause,
  BindingIdentifier,
  FunctionExpression,
} from '@romejs/js-ast';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {markup} from '@romejs/string-markup';

function extractBindingIdentifiers(
  node:
    | VariableDeclaration
    | FunctionDeclaration
    | FunctionExpression
    | CatchClause,
): BindingIdentifier[] {
  if (
    node.type === 'FunctionDeclaration' ||
    node.type === 'FunctionExpression'
  ) {
    const bindings = getBindingIdentifiers(node.head);

    if (node.id !== undefined) {
      bindings.push(node.id);
    }

    return bindings;
  }

  if (node.type === 'VariableDeclaration' || node.param) {
    return getBindingIdentifiers(node);
  }

  return [];
}

export default {
  name: 'noShadowRestrictedNames',
  enter(path: Path): TransformExitResult {
    const {node, context} = path;

    if (
      node.type === 'VariableDeclaration' ||
      node.type === 'FunctionDeclaration' ||
      node.type === 'FunctionExpression' ||
      node.type === 'CatchClause'
    ) {
      const bindings = extractBindingIdentifiers(node);

      for (const binding of bindings) {
        if (context.getRootScope().isGlobal(binding.name)) {
          context.addNodeDiagnostic(binding, {
            category: 'lint/noShadowRestrictedNames',
            message: markup`Shadowing of global property <emphasis>${binding.name}</emphasis>`,
          });
        }
      }
    }

    return node;
  },
};
