/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {
  ExportLocalDeclaration,
  exportLocalDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import ImportT from '../../types/ImportT';
import Hub from '../../Hub';

export default function ExportLocalDeclaration(
  node: AnyNode,
  scope: Scope,
  {evaluator}: Hub,
) {
  node = exportLocalDeclaration.assert(node);

  // export const foo = 'bar';

  // export default function foo() {}
  const decl = node.declaration;
  if (decl !== undefined) {
    const declType = scope.evaluate(decl);

    switch (decl.type) {
      case 'FunctionDeclaration':
      case 'ClassDeclaration':
        const id = decl.id;
        if (id === undefined) {
          throw new Error(`Expected id`);
        }
        evaluator.addExport(id.name, declType);
        break;

      case 'VariableDeclarationStatement':
        for (const id of getBindingIdentifiers(decl)) {
          const type = scope.getBinding(id.name);
          if (type === undefined) {
            throw new Error(`Couldn't find binding type for ${id.name}`);
          }
          evaluator.addExport(id.name, type);
        }
        break;

      case 'TypeAliasTypeAnnotation':
        const type = scope.getBinding(decl.id.name);
        if (type === undefined) {
          throw new Error(`Couldn't find binding type for ${decl.id.name}`);
        }
        evaluator.addExport(decl.id.name, type);
        break;
    }

    return declType;
  }

  // export {foo, bar};

  // export {foo, bar} from './foo';
  const source = undefined; // TODO node.source === undefined ? undefined : node.source.value;
  const {specifiers} = node;
  if (specifiers !== undefined) {
    for (const specifier of specifiers) {
      if (specifier.type === 'ExportLocalSpecifier' || specifier.type ===
      'ExportExternalSpecifier') {
        let type;
        if (source === undefined) {
          type = scope.evaluate(specifier.local);
        } else {
          type = new ImportT(scope, node, {
            importedName: specifier.local.name,
            source,
          });
        }
        evaluator.addExport(specifier.exported.name, type);
      }
    }
  }
}
