/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression, AnyStatement} from '../index';
import {createBuilder} from '../utils';

export type WithStatement = 
  & JSNodeBase
  & {
    type: 'WithStatement';
    object: AnyExpression;
    body: AnyStatement;
  };

export const withStatement = createBuilder<WithStatement>('WithStatement', {
  bindingKeys: {},
  visitorKeys: {
    object: true,
    body: true,
  },
});
