/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyExpression, SpreadElement} from '../index';
import {createQuickBuilder} from '../utils';

export type ArrayExpression = JSNodeBase & {
  type: 'ArrayExpression';
  elements: Array<undefined | AnyExpression | SpreadElement>;
};

export const arrayExpression = createQuickBuilder<ArrayExpression, 'elements'>(
  'ArrayExpression',
  'elements',
  {
    bindingKeys: {},
    visitorKeys: {
      elements: true,
    },
  },
);
