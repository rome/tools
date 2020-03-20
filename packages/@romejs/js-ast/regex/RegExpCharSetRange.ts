/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, AnyRegExpEscapedCharacter} from '../index';
import {createBuilder} from '../utils';

export type RegExpCharSetRange = 
  & JSNodeBase
  & {
    type: 'RegExpCharSetRange';
    start: AnyRegExpEscapedCharacter;
    end: AnyRegExpEscapedCharacter;
  };

export const regExpCharSetRange = createBuilder<RegExpCharSetRange>(
  'RegExpCharSetRange',
  {
    bindingKeys: {},
    visitorKeys: {
      start: true,
      end: true,
    },
  },
);
