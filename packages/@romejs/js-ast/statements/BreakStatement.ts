/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, Identifier} from '../index';
import {createBuilder} from '../utils';

export type BreakStatement = 
  & JSNodeBase
  & {
    type: 'BreakStatement';
    label?: Identifier;
  };

export const breakStatement = createBuilder<BreakStatement>('BreakStatement', {
  bindingKeys: {},
  visitorKeys: {
    label: true,
  },
});
