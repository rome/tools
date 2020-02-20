/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {stripAnsi} from './index';

const DEFAULT_SPACER = ' ';

export function pad(
  side: 'left' | 'right',
  str: string,
  len: number,
  spacerChar: string = DEFAULT_SPACER,
) {
  const stripped = stripAnsi(str);
  const remainder = len - stripped.length;

  if (remainder <= 0) {
    return str;
  }

  const spacer = spacerChar.repeat(remainder);
  if (side === 'left') {
    return spacer + str;
  } else {
    //right
    return str + spacer;
  }
}

export function rightPad(
  str: string,
  len: number,
  spacer: string = DEFAULT_SPACER,
) {
  return pad('right', str, len, spacer);
}

export function leftPad(
  str: string,
  len: number,
  spacer: string = DEFAULT_SPACER,
) {
  return pad('left', str, len, spacer);
}
