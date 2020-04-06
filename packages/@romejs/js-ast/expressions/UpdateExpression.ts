/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type UpdateExpression = JSNodeBase & {
  type: 'UpdateExpression';
  operator: UpdateOperator;
  argument: AnyExpression;
  prefix?: boolean;
};

export type UpdateOperator = '++' | '--';

export const updateExpression = createBuilder<UpdateExpression>(
  'UpdateExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      argument: true,
    },
  },
);
