/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  Identifier,
  ConstExportModuleKind,
  ReferenceIdentifier,
} from '../index';
import {createBuilder} from '../utils';

export type ExportLocalSpecifier = 
  & JSNodeBase
  & {
    type: 'ExportLocalSpecifier';
    exported: Identifier;
    local: ReferenceIdentifier;
    exportKind?: ConstExportModuleKind;
  };

export const exportLocalSpecifier = createBuilder<ExportLocalSpecifier>(
  'ExportLocalSpecifier',
  {
    bindingKeys: {},
    visitorKeys: {
      local: true,
      exported: true,
    },
  },
);
