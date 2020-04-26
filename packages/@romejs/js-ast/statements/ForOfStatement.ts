/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyExpression,
  AnyStatement,
  AnyTargetAssignmentPattern,
  JSNodeBase,
  VariableDeclaration,
} from '../index';
import {createBuilder} from '../utils';

export type ForOfStatement = JSNodeBase & {
  type: 'ForOfStatement';
  await?: boolean;
  left: VariableDeclaration | AnyTargetAssignmentPattern;
  right: AnyExpression;
  body: AnyStatement;
};

export const forOfStatement = createBuilder<ForOfStatement>(
  'ForOfStatement',
  {
    bindingKeys: {},
    visitorKeys: {
      left: true,
      right: true,
      body: true,
    },
  },
);
