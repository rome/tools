/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isTagStartChar} from './parse';
import {coerce0} from '@romejs/ob1';
import {Dict} from '@romejs/typescript-helpers';
import {MarkupTagName} from './types';
import {isEscaped} from '@romejs/parser-core';

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
    const isLast = i == input.length - 1;

    // Escape all <
    if (isTagStartChar(coerce0(i), input)) {
      escaped += '\\<';
    } else {
      // If this is the final character and we're a slash that is not escaped, ignore it.
      // We could have a tag after the end of this string that would be broken.
      if (isLast && char === '\\' && !isEscaped(coerce0(i - 1), input)) {
        continue;
      }

      escaped += char;
    }
  }
  return escaped;
}

export function markupTag(
  tagName: MarkupTagName,
  text: string,
  attrs?: Dict<string | number>,
): string {
  let ret = `<${tagName}`;

  if (attrs !== undefined) {
    for (const key in attrs) {
      const value = attrs[key];
      ret += markup` ${key}="${String(value)}"`;
    }
  }

  ret += `>${text}</${tagName}>`;

  return ret;
}
