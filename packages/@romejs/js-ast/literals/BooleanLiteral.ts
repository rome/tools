/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type BooleanLiteral =
  & JSNodeBase
  & {
    type: 'BooleanLiteral';
    value: boolean;
  };

export const booleanLiteral = createQuickBuilder<BooleanLiteral, 'value'>(
  'BooleanLiteral',
  'value',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
