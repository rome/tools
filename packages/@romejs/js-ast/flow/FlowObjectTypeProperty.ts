/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyFlowPrimary,
  Identifier,
  FlowVariance,
  StringLiteral,
  NumericLiteral,
} from '../index';
import {createBuilder} from '../utils';

export type FlowObjectTypePropertyKind = 'init' | 'get' | 'set';

export type FlowObjectTypePropertyKey =
  | Identifier
  | StringLiteral
  | NumericLiteral;

export type FlowObjectTypeProperty =
  & JSNodeBase
  & {
    type: 'FlowObjectTypeProperty';
    key: FlowObjectTypePropertyKey;
    value: AnyFlowPrimary;
    kind: FlowObjectTypePropertyKind;
    static?: boolean;
    optional?: boolean;
    proto?: boolean;
    variance?: FlowVariance;
  };

export const flowObjectTypeProperty = createBuilder<FlowObjectTypeProperty>(
  'FlowObjectTypeProperty',
  {
    bindingKeys: {},
    visitorKeys: {
      key: true,
      value: true,
      variance: true,
    },
  },
);
