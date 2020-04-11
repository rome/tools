/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression, JSXEmptyExpression} from '../index';
import {createBuilder} from '../utils';

export type JSXExpressionContainer = JSNodeBase & {
  type: 'JSXExpressionContainer';
  expression: AnyExpression | JSXEmptyExpression;
};

export const jsxExpressionContainer = createBuilder<JSXExpressionContainer>(
  'JSXExpressionContainer',
  {
    bindingKeys: {},
    visitorKeys: {
      expression: true,
    },
  },
);
