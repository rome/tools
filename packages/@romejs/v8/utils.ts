/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function hrTime(): number {
  const hrtime = process.hrtime(); // [seconds, nanoseconds]
  const ts = hrtime[0] * 1000000 + Math.round(hrtime[1] / 1000); // microseconds
  return ts;
}

const FILE_PROTOCOL = 'file://';

export function urlToFilename(url: string): string {
  if (url.startsWith(FILE_PROTOCOL)) {
    return url.slice(FILE_PROTOCOL.length);
  } else {
    return url;
  }
}
