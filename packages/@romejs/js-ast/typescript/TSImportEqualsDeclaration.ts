/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, BindingIdentifier, AnyTSModuleReference} from '../index';
import {createBuilder} from '../utils';

export type TSImportEqualsDeclaration = JSNodeBase & {
  type: 'TSImportEqualsDeclaration';
  id: BindingIdentifier;
  moduleReference: AnyTSModuleReference;
  isExport?: boolean;
};

export const tsImportEqualsDeclaration = createBuilder<TSImportEqualsDeclaration>(
  'TSImportEqualsDeclaration',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {id: true, moduleReference: true},
  },
);
