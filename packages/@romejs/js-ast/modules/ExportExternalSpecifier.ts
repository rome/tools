/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, Identifier, ConstExportModuleKind} from '../index';
import {createBuilder} from '../utils';

export type ExportExternalSpecifier =
  & JSNodeBase
  & {
    type: 'ExportExternalSpecifier';
    exported: Identifier;
    local: Identifier;
    exportKind?: ConstExportModuleKind;
  };

export const exportExternalSpecifier = createBuilder<ExportExternalSpecifier>(
  'ExportExternalSpecifier',
  {
    bindingKeys: {},
    visitorKeys: {
      exported: true,
      local: true,
    },
  },
);
