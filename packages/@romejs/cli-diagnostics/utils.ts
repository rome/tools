/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {markupToPlainTextString} from '@romejs/string-markup';
import highlightCode, {AnsiHighlightOptions} from './highlightCode';
import {NEWLINE} from '@romejs/js-parser-utils';

export function normalizeTabs(str: string): string {
  return str.replace(/\t/g, '  ');
}

export function showInvisibles(str: string): string {
  let ret = '';
  for (const cha of str) {
    switch (cha) {
      case ' ': {
        ret += '\xb7'; // Middle Dot, \u00B7
        break;
      }
      case '\r': {
        ret += '\u240d';
        break;
      }
      case '\n': {
        ret += '\u23ce'; // Return Symbol, \u23ce
        break;
      }
      case '\t': {
        ret += '\u21b9'; // Left Arrow To Bar Over Right Arrow To Bar, \u21b9
        break;
      }
      default: {
        ret += cha;
        break;
      }
    }
  }
  return ret;
}

export function cleanEquivalentString(str: string): string {
  str = markupToPlainTextString(str);

  // Replace all whitespace with spaces
  str = str.replace(/[\s\n]+/g, ' ');

  // Remove trailing dot
  str = str.replace(/\.+$/, '');

  // Remove surrounding quotes
  str = str.replace(/^"(.*?)"$/, '$1');

  return str;
}

export function joinNoBreak(lines: Array<string>): string {
  return `<nobr>${lines.join('\n')}</nobr>`;
}

export function splitLines(src: string): Array<string> {
  return src.replace(/\t/g, ' ').split(NEWLINE);
}

export type ToLines = {
  length: number;
  raw: Array<string>;
  highlighted: Array<string>;
};

export function toLines(opts: AnsiHighlightOptions): ToLines {
  const raw = splitLines(opts.input);
  const highlighted = splitLines(highlightCode(opts));

  if (raw.length !== highlighted.length) {
    throw new Error(
      `raw and highlighted line count mismatch ${raw.length} !== ${highlighted.length}`,
    );
  }

  return {
    length: raw.length,
    raw,
    highlighted,
  };
}
