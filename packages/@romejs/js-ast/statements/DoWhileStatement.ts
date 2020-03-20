/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyStatement, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type DoWhileStatement = 
  & JSNodeBase
  & {
    type: 'DoWhileStatement';
    body: AnyStatement;
    test: AnyExpression;
  };

export const doWhileStatement = createBuilder<DoWhileStatement>(
  'DoWhileStatement',
  {
    bindingKeys: {},
    visitorKeys: {
      test: true,
      body: true,
    },
  },
);
