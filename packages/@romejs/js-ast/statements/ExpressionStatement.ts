/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type ExpressionStatement =
  & JSNodeBase
  & {
    type: 'ExpressionStatement';
    expression: AnyExpression;
  };

export const expressionStatement = createBuilder<ExpressionStatement>(
  'ExpressionStatement',
  {
    bindingKeys: {},
    visitorKeys: {
      expression: true,
    },
  },
);
