/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type RegExpEndCharacter = JSNodeBase & {type: 'RegExpEndCharacter'};

export const regExpEndCharacter = createBuilder<RegExpEndCharacter>(
  'RegExpEndCharacter',
  {
    bindingKeys: {},
    visitorKeys: {},
  },
);
