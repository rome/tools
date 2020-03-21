/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {stripAnsi} from './format';
import {pattern} from './regex';

const startRegex = new RegExp(`^${pattern}`);

function isAnsiStartChar(char: string): boolean {
  return char === '\x1b' || char === '\x9b';
}

const HYPERLINK_END = '\x1b]8;;\x07';

function sliceAnsi(input: string, index: number): undefined | string {
  const match = input.slice(index).match(startRegex);

  if (match == null) {
    return undefined;
  }

  const str = match[0];

  // Hyperlink, try and find the rest
  if (str[0] === '\x1b') {
    const possibleIndex = input.indexOf(HYPERLINK_END, index + str.length);
    if (possibleIndex > -1) {
      return input.slice(index, possibleIndex + HYPERLINK_END.length);
    }
  }

  return str;
}

export function truncateAnsi(input: string, maxWidth: number): string {
  return splitAnsiLines(input, maxWidth)[0];
}

type MapAnsiStringItem = {
  isAnsi: boolean;
  start: number;
  end: number;
};

export function mapAnsiString(
  input: string,
  callback: (input: string, opts: MapAnsiStringItem) => string,
): string {
  let buff = '';
  let fakeIndex = 0;
  let realIndex = 0;

  while (realIndex < input.length) {
    const char = input[realIndex];

    // Skip all ansi sequences, but add them to the current line
    if (isAnsiStartChar(char)) {
      const match = sliceAnsi(input, realIndex);
      if (match !== undefined) {
        const strippedLength = stripAnsi(match).length;
        buff += callback(match, {
          isAnsi: true,
          start: fakeIndex,
          end: fakeIndex + strippedLength,
        });
        realIndex += match.length;
        fakeIndex += strippedLength;
        continue;
      }
    }

    buff += callback(char, {
      isAnsi: false,
      start: fakeIndex,
      end: fakeIndex + 1,
    });
    realIndex++;
    fakeIndex++;
  }

  return buff;
}

export function splitAnsiLines(input: string, maxWidth?: number): Array<string> {
  const lines: Array<string> = [];

  let column = 0;
  let buff = '';

  function pushLine() {
    lines.push(buff);
    column = 0;
    buff = '';
  }

  let index = 0;

  while (index < input.length) {
    const char = input[index];

    // Skip all ansi sequences, but add them to the current line
    if (isAnsiStartChar(char)) {
      const match = sliceAnsi(input, index);
      if (match !== undefined) {
        buff += match;
        index += match.length;
        continue;
      }
    }

    // Don't allow spaces at the beginning of lines
    if (char === ' ' && column === 0) {
      //index++;

      //continue;
    }

    // Flush the current line
    if (char === '\n') {
      pushLine();
      index++;
      continue;
    }

    // Otherwise, get all the characters until a space for soft wrapping
    let word = char;
    for (let i = index + 1; i < input.length && input[i] !== ' '; i++) {
      word += input[i];
    }

    // Calculate the word length, this can include ansi color codes
    const wordLength = stripAnsi(word).length;

    // Push the current line if the addition of this word would overflow
    let pushedLine = false;
    if (maxWidth !== undefined && column + wordLength > maxWidth) {
      pushLine();
      pushedLine = true;
    }

    // Add the word and advance
    column += wordLength;
    buff += pushedLine ? word.trimLeft() : word;
    index += word.length;
  }

  // Flush the current buffer
  if (buff !== '') {
    pushLine();
  }

  return lines;
}
