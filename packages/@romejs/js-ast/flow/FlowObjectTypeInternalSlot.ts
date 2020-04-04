/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyFlowPrimary, FlowObjectTypePropertyKey} from '../index';
import {createBuilder} from '../utils';

export type FlowObjectTypeInternalSlot = JSNodeBase & {
  type: 'FlowObjectTypeInternalSlot';
  id: FlowObjectTypePropertyKey;
  value: AnyFlowPrimary;
  optional?: boolean;
  static?: boolean;
  method?: boolean;
};

export const flowObjectTypeInternalSlot = createBuilder<
  FlowObjectTypeInternalSlot
>('FlowObjectTypeInternalSlot', {
  bindingKeys: {},
  visitorKeys: {
    id: true,
    value: true,
  },
});
