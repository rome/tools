/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  PatternMeta,
  AnyExpression,
  AnyTargetBindingPattern,
} from '../index';
import {createBuilder} from '../utils';

export type BindingAssignmentPattern =
  & JSNodeBase
  & {
    type: 'BindingAssignmentPattern';
    left: AnyTargetBindingPattern;
    right: AnyExpression;
    meta?: PatternMeta;
  };

export const bindingAssignmentPattern = createBuilder<BindingAssignmentPattern>(
  'BindingAssignmentPattern',
  {
    bindingKeys: {
      left: true,
    },
    visitorKeys: {
      left: true,
      right: true,
      meta: true,
    },
  },
);
