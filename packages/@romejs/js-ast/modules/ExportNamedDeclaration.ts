/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  VariableDeclarationStatement,
  FunctionDeclaration,
  ClassDeclaration,
  ExportSpecifier,
  StringLiteral,
  ConstExportModuleKind,
  ExportNamespaceSpecifier,
  ExportDefaultSpecifier,
  TSModuleDeclaration,
  TSEnumDeclaration,
  FlowInterfaceDeclaration,
  TypeAliasTypeAnnotation,
  TSInterfaceDeclaration,
  TSDeclareFunction,
  FlowOpaqueType,
} from '../index';
import {createBuilder} from '../utils';

export type AnyExportSpecifier =
  | ExportSpecifier
  | ExportNamespaceSpecifier
  | ExportDefaultSpecifier;

export type ExportNamedDeclaration = JSNodeBase & {
  type: 'ExportNamedDeclaration';
  declaration?:
    | undefined
    | VariableDeclarationStatement
    | FunctionDeclaration
    | ClassDeclaration
    | TSModuleDeclaration
    | TSEnumDeclaration
    | FlowInterfaceDeclaration
    | TypeAliasTypeAnnotation
    | TSInterfaceDeclaration
    | TSDeclareFunction
    | FlowOpaqueType
    | TypeAliasTypeAnnotation;
  specifiers?: Array<AnyExportSpecifier>;
  source?: StringLiteral;
  exportKind?: ConstExportModuleKind;
  declare?: boolean;
};

export const exportNamedDeclaration = createBuilder<ExportNamedDeclaration>(
  'ExportNamedDeclaration',
  {
    bindingKeys: {
      declaration: true,
    },
    visitorKeys: {
      declaration: true,
      specifiers: true,
      source: true,
    },
  },
);
