/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type UnaryExpression =
  & JSNodeBase
  & {
    type: 'UnaryExpression';
    operator: UnaryOperator;
    prefix?: boolean;
    argument: AnyExpression;
  };

export type UnaryOperator =
  | '-'
  | '+'
  | '!'
  | '~'
  | 'typeof'
  | 'void'
  | 'delete'
  | 'throw';

export const unaryExpression = createBuilder<UnaryExpression>(
  'UnaryExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      argument: true,
    },
  },
);
