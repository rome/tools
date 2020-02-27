/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function humanizeNumber(
  num: bigint | number,
  sep: string = '_',
): string {
  if (num < 1000) {
    return String(num);
  }

  const decimals = String(num).split('.');

  let intChars: Array<string> = String(decimals.shift()).split('');
  let intParts: Array<string> = [];

  while (intChars.length > 0) {
    const part = intChars.slice(-3).join('');
    intParts.unshift(part);

    intChars = intChars.slice(0, -3);
  }

  return [intParts.join(sep), ...decimals].join('.');
}
