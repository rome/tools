/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const ESC = '\x1b[';

export const escapes = {
  clearScreen: '\x1bc',
  eraseLine: `${ESC}2K`,

  cursorUp(count: number = 1): string {
    return `${ESC}${count}A`;
  },

  cursorTo(x: number, y?: number): string {
    if (y === undefined) {
      return `${ESC}${x + 1}G`;
    }

    return `${ESC}${y + 1};${x + 1}H`;
  },
};
