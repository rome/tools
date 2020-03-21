/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, Identifier, PrivateName} from '../index';
import {createQuickBuilder} from '../utils';

export type StaticMemberProperty =
  & JSNodeBase
  & {
    type: 'StaticMemberProperty';
    value: Identifier | PrivateName;
    optional?: boolean;
  };

export const staticMemberProperty = createQuickBuilder<
  StaticMemberProperty,
  'value'
>('StaticMemberProperty', 'value', {
  bindingKeys: {},
  visitorKeys: {
    value: true,
  },
});
