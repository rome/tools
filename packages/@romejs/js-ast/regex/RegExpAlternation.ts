/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRegExpExpression, JSNodeBase, RegExpSubExpression} from '../index';
import {createBuilder} from '../utils';

export type RegExpAlternation = JSNodeBase & {
  type: 'RegExpAlternation';
  left: AnyRegExpExpression;
  right: RegExpSubExpression;
};

export const regExpAlternation = createBuilder<RegExpAlternation>(
  'RegExpAlternation',
  {
    bindingKeys: {},
    visitorKeys: {
      left: true,
      right: true,
    },
  },
);
