/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  ExportExternalSpecifier,
  StringLiteral,
  ExportNamespaceSpecifier,
  ExportDefaultSpecifier,
  ConstExportModuleKind,
} from '../index';
import {createBuilder} from '../utils';

export type AnyExportExternalSpecifier =
  | ExportNamespaceSpecifier
  | ExportDefaultSpecifier
  | ExportExternalSpecifier;

export type ExportExternalDeclaration =
  & JSNodeBase
  & {
    type: 'ExportExternalDeclaration';
    defaultSpecifier?: ExportDefaultSpecifier;
    namespaceSpecifier?: ExportNamespaceSpecifier;
    namedSpecifiers: Array<ExportExternalSpecifier>;
    source: StringLiteral;
    exportKind?: ConstExportModuleKind;
  };

export const exportExternalDeclaration =
  createBuilder<ExportExternalDeclaration>('ExportExternalDeclaration', {
    bindingKeys: {},
    visitorKeys: {
      defaultSpecifier: true,
      namespaceSpecifier: true,
      namedSpecifiers: true,
      source: true,
    },
  });
