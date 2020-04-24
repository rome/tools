/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSEntityName, Identifier, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSQualifiedName = JSNodeBase & {
  type: 'TSQualifiedName';
  left: AnyTSEntityName;
  right: Identifier;
};

export const tsQualifiedName = createBuilder<TSQualifiedName>(
  'TSQualifiedName',
  {
    bindingKeys: {},
    visitorKeys: {
      left: true,
      right: true,
    },
  },
);
