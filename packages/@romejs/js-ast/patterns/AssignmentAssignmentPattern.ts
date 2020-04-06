/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  AnyTargetAssignmentPattern,
  AnyExpression,
  PatternMeta,
} from '../index';
import {createBuilder} from '../utils';

export type AssignmentAssignmentPattern = JSNodeBase & {
  type: 'AssignmentAssignmentPattern';
  left: AnyTargetAssignmentPattern;
  right: AnyExpression;
  meta?: PatternMeta;
};

export const assignmentAssignmentPattern = createBuilder<
  AssignmentAssignmentPattern
>('AssignmentAssignmentPattern', {
  bindingKeys: {},
  visitorKeys: {
    left: true,
    right: true,
    meta: true,
  },
});
