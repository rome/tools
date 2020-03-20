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
  AnyTargetAssignmentPattern,
  AnyStatement,
} from '../index';
import {createBuilder} from '../utils';

export type ForInStatement =
  & JSNodeBase
  & {
    type: 'ForInStatement';
    left: VariableDeclaration | AnyTargetAssignmentPattern;
    right: AnyExpression;
    body: AnyStatement;
  };

export const forInStatement = createBuilder<ForInStatement>('ForInStatement', {
  bindingKeys: {},
  visitorKeys: {
    left: true,
    right: true,
    body: true,
  },
});
