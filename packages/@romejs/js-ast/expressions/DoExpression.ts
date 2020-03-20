/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, BlockStatement} from '../index';
import {createBuilder} from '../utils';

export type DoExpression =
  & JSNodeBase
  & {
    type: 'DoExpression';
    body: BlockStatement;
  };

export const doExpression = createBuilder<DoExpression>('DoExpression', {
  bindingKeys: {},
  visitorKeys: {
    body: true,
  },
});
