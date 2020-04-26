/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyStatement, VariableDeclarator} from '@romejs/js-ast';
import {ConstBinding, Path} from '@romejs/js-compiler';
import {getRequireSource, isInTypeAnnotation} from '@romejs/js-ast-utils';

const NON_INLINED_REQUIRES: Array<string> = [];

export default {
  name: 'inlineRequiresTransform',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'ReferenceIdentifier') {
      const binding = path.scope.getBinding(node.name);

      // Inline references to a require variable
      if (binding !== undefined && binding instanceof ConstBinding) {
        const source = getRequireSource(binding.value, path.scope, true);
        if (
          source !== undefined &&
          !NON_INLINED_REQUIRES.includes(source) &&
          !isInTypeAnnotation(path) &&
          binding.value !== undefined
        ) {
          return binding.value;
        }
      }
    }

    return node;
  },
  exit(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'Program' || node.type === 'BlockStatement') {
      const body: Array<AnyStatement> = [];
      let hadRequires = false;

      // Remove all require declarations that could have been inlined
      for (const bodyNode of node.body) {
        if (
          bodyNode.type === 'VariableDeclarationStatement' &&
          bodyNode.declaration.kind === 'const'
        ) {
          let hadRequireDeclarators = false;
          const declarators: Array<VariableDeclarator> = [];

          for (const decl of bodyNode.declaration.declarations) {
            if (decl.id.type !== 'BindingIdentifier') {
              // Patterns aren't supported yet
              declarators.push(decl);
              continue;
            }

            const source = getRequireSource(decl.init, path.scope, true);
            if (source === undefined) {
              // Didn't contain a `require`
              declarators.push(decl);
              continue;
            }

            if (NON_INLINED_REQUIRES.includes(source)) {
              // Blacklisted
              declarators.push(decl);
              continue;
            }

            hadRequireDeclarators = true;
            hadRequires = true;
          }

          if (hadRequireDeclarators) {
            if (declarators.length > 0) {
              body.push({
                ...bodyNode,
                declaration: {
                  ...bodyNode.declaration,
                  declarations: declarators,
                },
              });
            }
            continue;
          }
        }

        body.push(bodyNode);
      }

      if (!hadRequires) {
        return node;
      }

      return {
        ...node,
        body,
      };
    }

    return node;
  },
};
