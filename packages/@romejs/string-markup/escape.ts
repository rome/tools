/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isTagChar} from './parse';
import {coerce0} from '@romejs/ob1';

// A tagged template literal helper that will escape all interpolated strings, ensuring only markup works
export function markup(
  strs: TemplateStringsArray,
...values: Array<unknown>): string {
  let out = '';

  for (let i = 0; i < strs.length; i++) {
    const str = strs[i];
    out += str;

    const interpolated = values[i];

    if (interpolated instanceof SafeMarkup) {
      out += interpolated.value;
      continue;
    }

    if (interpolated !== undefined) {
      out += escapeMarkup(String(interpolated));
    }
  }

  return out;
}

class SafeMarkup {
  constructor(value: string) {
    this.value = value;
  }

  value: string;
}

export function safeMarkup(input: string): SafeMarkup {
  return new SafeMarkup(input);
}

export function escapeMarkup(input: string): string {
  let escaped = '';
  for (let i = 0; i < input.length; i++) {
    const char = input[i];

    if (isTagChar(coerce0(i), input)) {
      escaped += '\\<';
    } else {
      escaped += char;
    }
  }
  return escaped;
}
