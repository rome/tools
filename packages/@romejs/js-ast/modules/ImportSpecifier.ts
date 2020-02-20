/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, Identifier, ImportSpecifierLocal} from '../index';
import {createBuilder} from '../utils';

export type ImportSpecifier = JSNodeBase & {
  type: 'ImportSpecifier';
  imported: Identifier;
  local: ImportSpecifierLocal;
};

export const importSpecifier = createBuilder<ImportSpecifier>(
  'ImportSpecifier',
  {
    bindingKeys: {
      local: true,
    },
    visitorKeys: {
      imported: true,
      local: true,
    },
  },
);
