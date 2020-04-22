/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Tokens, operator, word} from '../../tokens';

export default function DebuggerStatement(): Tokens {
  return [word('debugger'), operator(';')];
}
