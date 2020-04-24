/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTargetAssignmentPattern, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSAssignmentNonNullExpression = JSNodeBase & {
  type: 'TSAssignmentNonNullExpression';
  expression: AnyTargetAssignmentPattern;
};

export const tsAssignmentNonNullExpression = createBuilder<
  TSAssignmentNonNullExpression
>('TSAssignmentNonNullExpression', {
  bindingKeys: {},
  visitorKeys: {expression: true},
});
