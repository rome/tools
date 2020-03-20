/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type RegExpNonDigitCharacter = 
  & JSNodeBase
  & {type: 'RegExpNonDigitCharacter'};

export const regExpNonDigitCharacter = createBuilder<RegExpNonDigitCharacter>(
  'RegExpNonDigitCharacter',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
