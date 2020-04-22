/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyPrimaryType, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type FlowTypeCastExpression = JSNodeBase & {
  type: 'FlowTypeCastExpression';
  expression: AnyExpression;
  typeAnnotation: AnyPrimaryType;
};

export const flowTypeCastExpression = createBuilder<FlowTypeCastExpression>(
  'FlowTypeCastExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      expression: true,
      typeAnnotation: true,
    },
  },
);
