/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {stripAnsi} from '@romejs/string-ansi';
import ansiHighlightCode, {AnsiHighlightOptions} from './ansiHighlightCode';
import {NEWLINE} from '@romejs/js-parser-utils';

export function showInvisibles(str: string): string {
  let ret = '';
  for (const cha of str) {
    switch (cha) {
      case ' ':
        ret += '·'; // Middle Dot, \u00B7
        break;
      case '\n':
        ret += '⏎\n'; // Return Symbol, \u23ce
        break;
      case '\t':
        ret += '↹'; // Left Arrow To Bar Over Right Arrow To Bar, \u21b9
        break;
      default:
        ret += cha;
        break;
    }
  }
  return ret;
}

export function cleanEquivalentString(str: string): string {
  // Strip ansi
  str = stripAnsi(str);

  // Replace all whitespace with spaces
  str = str.replace(/[\s\n]+/g, ' ');

  // Remove trailing dot
  str = str.replace(/\.+$/, '');

  // Remove surrounding quotes
  str = str.replace(/^"(.*?)"$/, '$1');

  return str;
}

export function splitLines(src: string): Array<string> {
  return src.replace(/\t/g, ' ').split(NEWLINE);
}

export function toLines(opts: AnsiHighlightOptions): Array<string> {
  const highlighted = ansiHighlightCode(opts);
  const lines = splitLines(highlighted);
  return lines;
}
