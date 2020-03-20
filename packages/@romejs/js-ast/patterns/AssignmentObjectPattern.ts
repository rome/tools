/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  PatternMeta,
  AssignmentObjectPatternProperty,
  AssignmentIdentifier,
} from '../index';
import {createBuilder} from '../utils';

export type AssignmentObjectPattern = 
  & JSNodeBase
  & {
    type: 'AssignmentObjectPattern';
    meta?: PatternMeta;
    properties: Array<AssignmentObjectPatternProperty>;
    rest: undefined | AssignmentIdentifier;
  };

export const assignmentObjectPattern = createBuilder<AssignmentObjectPattern>(
  'AssignmentObjectPattern',
  {
    bindingKeys: {},
    visitorKeys: {
      properties: true,
      rest: true,
      meta: true,
    },
  },
);
