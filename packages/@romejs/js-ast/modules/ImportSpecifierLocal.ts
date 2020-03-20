/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, BindingIdentifier, ConstImportModuleKind} from '../index';
import {createQuickBuilder} from '../utils';

export type ImportSpecifierLocal =
  & JSNodeBase
  & {
    type: 'ImportSpecifierLocal';
    name: BindingIdentifier;
    importKind?: ConstImportModuleKind;
  };

export const importSpecifierLocal = createQuickBuilder<
  ImportSpecifierLocal,
  'name'
>('ImportSpecifierLocal', 'name', {
  bindingKeys: {
    name: true,
  },
  visitorKeys: {
    name: true,
  },
});
