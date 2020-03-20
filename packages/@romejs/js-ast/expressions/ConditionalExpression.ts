/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type ConditionalExpression =
  & JSNodeBase
  & {
    type: 'ConditionalExpression';
    test: AnyExpression;
    alternate: AnyExpression;
    consequent: AnyExpression;
  };

export const conditionalExpression = createBuilder<ConditionalExpression>(
  'ConditionalExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      test: true,
      consequent: true,
      alternate: true,
    },
  },
);
