/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyTSPrimary} from '../index';
import {createBuilder} from '../utils';

export type TSArrayType =
  & JSNodeBase
  & {
    type: 'TSArrayType';
    elementType: AnyTSPrimary;
  };

export const tsArrayType = createBuilder<TSArrayType>('TSArrayType', {
  bindingKeys: {},
  visitorKeys: {elementType: true},
});
