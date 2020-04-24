/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyExpression,
  AnyTypeArguments,
  JSNodeBase,
  SpreadElement,
  Super,
} from '../index';
import {createBuilder} from '../utils';

export type CallExpression = JSNodeBase & {
  type: 'CallExpression';
  callee: AnyExpression | Super;
  arguments: Array<AnyExpression | SpreadElement>;
  typeArguments?: AnyTypeArguments;
};

export const callExpression = createBuilder<CallExpression>('CallExpression', {
  bindingKeys: {},
  visitorKeys: {
    callee: true,
    arguments: true,
    typeArguments: true,
  },
});
