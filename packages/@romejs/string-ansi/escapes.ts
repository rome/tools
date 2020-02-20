/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const ESC = '\u001B[';

export const escapes = {
  clearScreen: '\u001Bc',
  eraseLine: `${ESC}2K`,

  cursorTo(x: number, y?: number): string {
    if (y === undefined) {
      return `${ESC}${x + 1}G`;
    }

    return `${ESC}${y + 1};${x + 1}H`;
  },
};
