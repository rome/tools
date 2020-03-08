/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type RegExpWhiteSpaceCharacter = JSNodeBase & {
  type: 'RegExpWhiteSpaceCharacter';
};

export const regExpWhiteSpaceCharacter = createBuilder<
  RegExpWhiteSpaceCharacter
>('RegExpWhiteSpaceCharacter', {
  bindingKeys: {},
  visitorKeys: {},
});
