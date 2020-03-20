/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, BindingIdentifier, FunctionHead} from '../index';
import {createBuilder} from '../utils';
import {BlockStatement} from '../statements/BlockStatement';

export type FunctionExpression = 
  & JSNodeBase
  & {
    type: 'FunctionExpression';
    id?: BindingIdentifier;
    head: FunctionHead;
    body: BlockStatement;
  };

export const functionExpression = createBuilder<FunctionExpression>(
  'FunctionExpression',
  {
    bindingKeys: {
      id: true,
    },
    visitorKeys: {
      head: true,
      id: true,
      body: true,
    },
  },
);
