/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyFlowPrimary,
  AnyPrimaryType,
  FlowVariance,
} from '../index';
import {createBuilder} from '../utils';

export type FlowTypeParameter = JSNodeBase & {
  type: 'FlowTypeParameter';
  name: string;
  bound?: AnyPrimaryType;
  default?: AnyFlowPrimary;
  variance?: FlowVariance;
};

export const flowTypeParameter = createBuilder<FlowTypeParameter>(
  'FlowTypeParameter',
  {
    bindingKeys: {},
    visitorKeys: {
      bound: true,
      default: true,
      variance: true,
    },
  },
);
