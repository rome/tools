/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyAssignmentPattern, AnyExpression} from '../index';
import {createBuilder} from '../utils';

export type AssignmentExpression = 
  & JSNodeBase
  & {
    type: 'AssignmentExpression';
    operator: AssignmentOperator;
    left: AnyAssignmentPattern;
    right: AnyExpression;
  };

export type AssignmentOperator = 
    | '='
    | '+='
    | '-='
    | '*='
    | '/='
    | '%='
    | '<<='
    | '>>='
    | '>>>='
    | '|='
    | '^='
    | '&='
    | '??=';

export const assignmentExpression = createBuilder<AssignmentExpression>(
  'AssignmentExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      left: true,
      right: true,
    },
  },
);
