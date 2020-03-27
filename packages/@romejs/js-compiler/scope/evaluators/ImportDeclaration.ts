/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {ImportBindingMeta, ImportBinding} from '@romejs/js-compiler';
import {ImportDeclaration, AnyNode, ConstImportModuleKind} from '@romejs/js-ast';
import {getImportSpecifiers} from '@romejs/js-ast-utils';

export default {
  creator: false,
  build(node: ImportDeclaration, parent: AnyNode, scope: Scope) {
    const source = node.source.value;

    for (const specifier of getImportSpecifiers(node)) {
      let kind: ConstImportModuleKind = specifier.local.importKind ||
      node.importKind || 'value';
      let meta: undefined | ImportBindingMeta;

      if (specifier.type === 'ImportNamespaceSpecifier') {
        meta = {
          kind,
          type: 'namespace',
          source,
        };
      } else if (specifier.type === 'ImportDefaultSpecifier') {
        meta = {
          kind,
          type: 'name',
          imported: 'default',
          source,
        };
      } else if (specifier.type === 'ImportSpecifier') {
        meta = {
          kind,
          type: 'name',
          imported: specifier.imported.name,
          source,
        };
      }

      if (meta === undefined) {
        return undefined;
      }

      scope.addBinding(new ImportBinding({
        node: specifier.local.name,
        name: specifier.local.name.name,
        scope,
      }, meta));
    }
  },
};
