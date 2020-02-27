/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  ExportDefaultDeclaration,
  ClassDeclaration,
  FunctionDeclaration,
} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

const DEFAULT_BASENAME_MESSAGE =
  'When exporting a default value with an id, the filename should be the same. eg. `export ' +
  'default class Foo {}` should be named `Foo.js`';

function isValidDeclaration(
  node: AnyNode,
): node is FunctionDeclaration | ClassDeclaration {
  return (
    node.type === 'FunctionDeclaration' || node.type === 'ClassDeclaration'
  );
}

export default {
  name: 'defaultExportSameBasename',
  enter(path: Path): AnyNode {
    const {context, node: program} = path;

    if (program.type === 'Program') {
      // Find the default export
      let defaultExport: undefined | ExportDefaultDeclaration;
      for (const node of program.body) {
        if (node.type === 'ExportDefaultDeclaration') {
          defaultExport = node;
          break;
        }
      }

      // Validate the export
      if (
        defaultExport !== undefined &&
        isValidDeclaration(defaultExport.declaration)
      ) {
        // Get the export default id
        const id = defaultExport.declaration.id;
        if (id !== undefined && context.path !== undefined) {
          const basename = context.path.getExtensionlessBasename();
          if (basename !== id.name && basename !== 'index') {
            context.addNodeDiagnostic(program, {
              category: 'lint/defaultExportSameBasename',
              message: DEFAULT_BASENAME_MESSAGE,
            });
          }
        }
      }
    }

    return program;
  },
};
