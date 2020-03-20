/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyTSPrimary, AnyTargetAssignmentPattern} from '../index';
import {createBuilder} from '../utils';

export type TSAssignmentAsExpression =
  & JSNodeBase
  & {
    type: 'TSAssignmentAsExpression';
    typeAnnotation: AnyTSPrimary;
    expression: AnyTargetAssignmentPattern;
  };

export const tsAssignmentAsExpression = createBuilder<TSAssignmentAsExpression>(
  'TSAssignmentAsExpression',
  {
    bindingKeys: {},
    visitorKeys: {expression: true, typeAnnotation: true},
  },
);
