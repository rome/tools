/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyAssignmentPattern, AnyObjectPropertyKey, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type AssignmentObjectPatternProperty = JSNodeBase & {
  type: 'AssignmentObjectPatternProperty';
  key: AnyObjectPropertyKey;
  value: AnyAssignmentPattern;
};

export const assignmentObjectPatternProperty = createBuilder<AssignmentObjectPatternProperty>(
  'AssignmentObjectPatternProperty',
  {
    bindingKeys: {},
    visitorKeys: {
      key: true,
      value: true,
    },
  },
);
