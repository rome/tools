/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  Super,
  AnyExpression,
  SpreadElement,
  FlowTypeParameterInstantiation,
  TSTypeParameterInstantiation,
} from '../index';
import {createBuilder} from '../utils';

export type NewExpression = 
  & JSNodeBase
  & {
    type: 'NewExpression';
    callee: AnyExpression | Super;
    arguments: Array<AnyExpression | SpreadElement>;
    typeArguments?: 
        | undefined
        | FlowTypeParameterInstantiation
        | TSTypeParameterInstantiation;
    optional?: boolean;
  };

export const newExpression = createBuilder<NewExpression>('NewExpression', {
  bindingKeys: {},
  visitorKeys: {
    callee: true,
    arguments: true,
    typeArguments: true,
  },
});
