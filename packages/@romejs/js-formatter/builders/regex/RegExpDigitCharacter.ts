/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Tokens, verbatim} from '../../tokens';
import {RegExpDigitCharacter} from '@romejs/js-ast';

export default function RegExpDigitCharacter(): Tokens {
  return [verbatim('\\d')];
}
