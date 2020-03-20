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
  ExportLocalSpecifier,
  ConstExportModuleKind,
  TSModuleDeclaration,
  TSEnumDeclaration,
  FlowInterfaceDeclaration,
  TypeAliasTypeAnnotation,
  TSInterfaceDeclaration,
  TSDeclareFunction,
  FlowOpaqueType,
} from '../index';
import {createBuilder} from '../utils';

export type ExportLocalDeclaration =
  & JSNodeBase
  & {
    type: 'ExportLocalDeclaration';
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
    specifiers?: Array<ExportLocalSpecifier>;
    exportKind?: ConstExportModuleKind;
    declare?: boolean;
  };

export const exportLocalDeclaration = createBuilder<ExportLocalDeclaration>(
  'ExportLocalDeclaration',
  {
    bindingKeys: {
      declaration: true,
    },
    visitorKeys: {
      declaration: true,
      specifiers: true,
    },
  },
);
