/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, ImportDeclaration} from '@romejs/js-ast';

export const foo = 'bar';
export default {
  name: 'duplicateImport',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'Program') {
      let duplicateImports: String[] = [];
      const filteredImports = (node.body.filter(
        node => node.type === 'ImportDeclaration',
      ) as ImportDeclaration[]).reduce<String[]>((acc, cur) => {
        if (acc.includes(cur.source.value)) {
          if (!duplicateImports.includes(cur.source.value)) {
            duplicateImports = duplicateImports.concat(` ${cur.source.value}`);
          }
          return acc;
        } else {
          return [...acc, cur.source.value];
        }
      }, []);
      if (
        filteredImports.length <
        node.body.filter(node => node.type === 'ImportDeclaration').length
      ) {
        path.context.addNodeDiagnostic(node, {
          category: 'lint/duplicateImport',
          message: `You are importing from the the following sources more than once:
              ${duplicateImports}`,
        });
      }
    }

    return node;
  },
};
