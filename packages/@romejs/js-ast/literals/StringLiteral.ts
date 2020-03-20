/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type StringLiteral =
  & JSNodeBase
  & {
    type: 'StringLiteral';
    value: string;
  };

export const stringLiteral = createQuickBuilder<StringLiteral, 'value'>(
  'StringLiteral',
  'value',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
