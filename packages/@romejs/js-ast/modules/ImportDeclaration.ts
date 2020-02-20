/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ConstImportModuleKind,
  JSNodeBase,
  StringLiteral,
  ImportSpecifier,
  ImportDefaultSpecifier,
  ImportNamespaceSpecifier,
} from '../index';
import {createBuilder} from '../utils';

export type AnyImportSpecifier =
  | ImportSpecifier
  | ImportDefaultSpecifier
  | ImportNamespaceSpecifier;

export type ImportDeclaration = JSNodeBase & {
  type: 'ImportDeclaration';
  specifiers?: Array<AnyImportSpecifier>;
  source: StringLiteral;
  importKind?: ConstImportModuleKind;
};

export const importDeclaration = createBuilder<ImportDeclaration>(
  'ImportDeclaration',
  {
    bindingKeys: {},
    visitorKeys: {
      specifiers: true,
      source: true,
    },
  },
);
