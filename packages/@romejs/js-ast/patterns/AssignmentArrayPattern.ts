/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  PatternMeta,
  AnyAssignmentPattern,
  AnyTargetAssignmentPattern,
} from '../index';
import {createBuilder} from '../utils';

export type AssignmentArrayPattern = 
  & JSNodeBase
  & {
    type: 'AssignmentArrayPattern';
    meta?: PatternMeta;
    elements: Array<undefined | AnyAssignmentPattern>;
    rest?: AnyTargetAssignmentPattern;
  };

export const assignmentArrayPattern = createBuilder<AssignmentArrayPattern>(
  'AssignmentArrayPattern',
  {
    bindingKeys: {},
    visitorKeys: {
      elements: true,
      rest: true,
      meta: true,
    },
  },
);
