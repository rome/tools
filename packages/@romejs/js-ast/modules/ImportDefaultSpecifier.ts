/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, ImportSpecifierLocal} from '../index';
import {createBuilder} from '../utils';

export type ImportDefaultSpecifier =
  & JSNodeBase
  & {
    type: 'ImportDefaultSpecifier';
    local: ImportSpecifierLocal;
  };

export const importDefaultSpecifier = createBuilder<ImportDefaultSpecifier>(
  'ImportDefaultSpecifier',
  {
    bindingKeys: {
      local: true,
    },
    visitorKeys: {
      local: true,
    },
  },
);
