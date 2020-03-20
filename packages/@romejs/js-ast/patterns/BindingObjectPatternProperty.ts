/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyObjectPropertyKey, AnyBindingPattern} from '../index';
import {createBuilder} from '../utils';

export type BindingObjectPatternProperty = 
  & JSNodeBase
  & {
    type: 'BindingObjectPatternProperty';
    key: AnyObjectPropertyKey;
    value: AnyBindingPattern;
    meta?: undefined;
  };

export const bindingObjectPatternProperty = createBuilder<
  BindingObjectPatternProperty
>('BindingObjectPatternProperty', {
  bindingKeys: {
    value: true,
  },
  visitorKeys: {
    key: true,
    value: true,
  },
});
