/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type ComputedPropertyKey = JSNodeBase & {
  type: 'ComputedPropertyKey';
  value: AnyExpression;
};

export const computedPropertyKey = createQuickBuilder<
  ComputedPropertyKey,
  'value'
>(
  'ComputedPropertyKey',
  'value',
  {
    bindingKeys: {},
    visitorKeys: {
      value: true,
    },
  },
);
