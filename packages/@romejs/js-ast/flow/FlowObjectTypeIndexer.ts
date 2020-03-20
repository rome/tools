/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyFlowPrimary,
  FlowObjectTypePropertyKey,
  FlowVariance,
} from '../index';
import {createBuilder} from '../utils';

export type FlowObjectTypeIndexer = 
  & JSNodeBase
  & {
    type: 'FlowObjectTypeIndexer';
    id?: FlowObjectTypePropertyKey;
    key: AnyFlowPrimary;
    value: AnyFlowPrimary;
    static?: boolean;
    variance?: FlowVariance;
  };

export const flowObjectTypeIndexer = createBuilder<FlowObjectTypeIndexer>(
  'FlowObjectTypeIndexer',
  {
    bindingKeys: {},
    visitorKeys: {
      id: true,
      key: true,
      value: true,
      variance: true,
    },
  },
);
