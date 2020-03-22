/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, ConstExportModuleKind, StringLiteral} from '../index';
import {createBuilder} from '../utils';

export type ExportAllDeclaration =
  & JSNodeBase
  & {
    type: 'ExportAllDeclaration';
    source: StringLiteral;
    exportKind?: ConstExportModuleKind;
    declare?: boolean;
  };

export const exportAllDeclaration = createBuilder<ExportAllDeclaration>(
  'ExportAllDeclaration',
  {
    bindingKeys: {},
    visitorKeys: {
      source: true,
    },
  },
);
