/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type NumericLiteral = JSNodeBase & {
  type: 'NumericLiteral';
  value: number;
  format?: 'octal' | 'binary' | 'hex';
};

export const numericLiteral = createQuickBuilder<NumericLiteral, 'value'>(
  'NumericLiteral',
  'value',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
