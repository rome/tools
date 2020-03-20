/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, PatternMeta} from '../index';
import {createQuickBuilder} from '../utils';

export type ReferenceIdentifier = 
  & JSNodeBase
  & {
    type: 'ReferenceIdentifier';
    name: string;
    definite?: boolean;
    meta?: PatternMeta;
  };

export const referenceIdentifier =
createQuickBuilder<ReferenceIdentifier, 'name'>('ReferenceIdentifier', 'name', {
  bindingKeys: {},
  visitorKeys: {
    meta: true,
  },
});
