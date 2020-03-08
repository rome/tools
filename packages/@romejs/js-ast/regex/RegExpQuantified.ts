/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyRegExpBodyItem} from '../index';
import {createBuilder} from '../utils';

export type RegExpQuantified = JSNodeBase & {
  type: 'RegExpQuantified';
  item: AnyRegExpBodyItem;
  lazy?: boolean;
  min: number;
  max?: number;
};

export const regExpQuantified = createBuilder<RegExpQuantified>(
  'RegExpQuantified',
  {
    bindingKeys: {},
    visitorKeys: {
      item: true,
    },
  },
);
