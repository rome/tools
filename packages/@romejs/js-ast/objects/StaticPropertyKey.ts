/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  FlowVariance,
  Identifier,
  PrivateName,
  StringLiteral,
  NumericLiteral,
} from '../index';
import {createQuickBuilder} from '../utils';

export type StaticPropertyKey =
  & JSNodeBase
  & {
    type: 'StaticPropertyKey';
    variance?: FlowVariance;
    value: Identifier | PrivateName | StringLiteral | NumericLiteral;
  };

export const staticPropertyKey = createQuickBuilder<StaticPropertyKey, 'value'>(
  'StaticPropertyKey',
  'value',
  {
    bindingKeys: {},
    visitorKeys: {
      variance: true,
      value: true,
    },
  },
);
