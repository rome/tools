/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Tokens} from '../../tokens';
import {word} from '@romejs/js-formatter/tokens';

export default function NullLiteral(): Tokens {
  return [word('null')];
}
