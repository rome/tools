/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyExportSpecifier,
  StringLiteral,
  ConstExportModuleKind,
  AnyFlowDeclare,
  ExportNamedDeclaration,
} from '../index';
import {createBuilder} from '../utils';

export type FlowDeclareExportNamed = JSNodeBase & {
  type: 'FlowDeclareExportNamed';
  declaration?: AnyFlowDeclare | ExportNamedDeclaration['declaration'];
  specifiers?: Array<AnyExportSpecifier>;
  source?: StringLiteral;
  exportKind?: ConstExportModuleKind;
  declare?: boolean;
};

export const flowDeclareExportNamed = createBuilder<FlowDeclareExportNamed>(
  'FlowDeclareExportNamed',
  {
    bindingKeys: {},
    visitorKeys: {
      specifiers: true,
      declaration: true,
      source: true,
    },
  },
);
