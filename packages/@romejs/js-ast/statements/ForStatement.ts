/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  VariableDeclaration,
  AnyExpression,
  AnyStatement,
} from '../index';
import {createBuilder} from '../utils';

export type ForStatement =
  & JSNodeBase
  & {
    type: 'ForStatement';
    init?: VariableDeclaration | AnyExpression;
    test?: AnyExpression;
    update?: AnyExpression;
    body: AnyStatement;
  };

export const forStatement = createBuilder<ForStatement>('ForStatement', {
  bindingKeys: {},
  visitorKeys: {
    init: true,
    test: true,
    update: true,
    body: true,
  },
});
