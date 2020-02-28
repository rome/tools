/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
  AnyNode,
  ExportLocalDeclaration,
  ExportExternalDeclaration,
} from '@romejs/js-ast';
import {ImportBinding} from '@romejs/js-compiler';
import {
  exportLocalDeclaration,
  exportExternalDeclaration,
  exportExternalSpecifier,
  identifier,
  stringLiteral,
} from '@romejs/js-ast';

export default {
  name: 'optimizeExports',
  enter(
    path: Path,
  ): AnyNode | Array<ExportExternalDeclaration | ExportLocalDeclaration> {
    const {node} = path;

    // turn `import {a} from 'b'; export {a}`; to `export {a} from 'b';`';
    if (
      node.type === 'ExportLocalDeclaration' &&
      node.exportKind === 'value' &&
      node.declaration === undefined &&
      node.specifiers !== undefined
    ) {
      const nodes: Array<
        ExportExternalDeclaration | ExportLocalDeclaration
      > = [];
      const specifiers = [];

      for (const specifier of node.specifiers) {
        if (specifier.type === 'ExportLocalSpecifier') {
          const binding = path.scope.getBinding(specifier.local.name);
          if (
            binding !== undefined &&
            binding instanceof ImportBinding &&
            binding.meta.type === 'name'
          ) {
            nodes.push(
              exportExternalDeclaration.create({
                specifiers: [
                  exportExternalSpecifier.create({
                    local: identifier.quick(binding.meta.imported),
                    exported: specifier.exported,
                    loc: specifier.loc,
                  }),
                ],
                source: stringLiteral.quick(binding.meta.source),
              }),
            );
          } else {
            specifiers.push(specifier);
          }
        } else {
          // TODO ???
          specifiers.push(specifier);
        }
      }

      if (specifiers.length === node.specifiers.length && nodes.length === 0) {
        return node;
      }

      if (specifiers.length !== 0) {
        nodes.push(exportLocalDeclaration.create({specifiers}));
      }

      return nodes;
    }

    return node;
  },
};
