/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const ESC = '\x1b[';

export const pattern = [
  '[\\u001B\\u009B][[\\]()#;?]*(?:(?:(?:[a-zA-Z\\d]*(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]*)*)?\\u0007)',
  '(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PR-TZcf-ntqry=><~]))',
].join('|');

export const regex = new RegExp(pattern, 'g');

function createEscape(num: number): string {
  return `${ESC}${String(num)}m`;
}

export const formatAnsi = {
  reset(str: string): string {
    return createEscape(0) + str + createEscape(0);
  },
  fileHyperlink(name: string, filename: string): string {
    let href = `file://`;
    const {HOSTNAME} = process.env;
    if (HOSTNAME != null) {
      href += `${HOSTNAME}/`;
    }
    href += filename;
    return formatAnsi.hyperlink(name, href);
  },
  hyperlink(name: string, href: string): string {
    return `\u001b]8;;${href}\u0007${name}\u001b]8;;\u0007`;
  },
  rgb(
    str: string,
    color: {
      r: number;
      g: number;
      b: number;
    },
  ): string {
    return (
      `\u001b[38;2;${String(color.r)};${String(color.g)};${String(color.b)}m` +
      str +
      createEscape(39)
    );
  },
  bgRgb(
    str: string,
    color: {
      r: number;
      g: number;
      b: number;
    },
  ): string {
    return (
      `\u001b[48;2;${String(color.r)};${String(color.g)};${String(color.b)}m` +
      str +
      createEscape(49)
    );
  },
  bold(str: string): string {
    return createEscape(1) + str + createEscape(22);
  },
  dim(str: string): string {
    return createEscape(2) + str + createEscape(22);
  },
  italic(str: string): string {
    return createEscape(3) + str + createEscape(23);
  },
  underline(str: string): string {
    return createEscape(4) + str + createEscape(24);
  },
  inverse(str: string): string {
    return createEscape(7) + str + createEscape(27);
  },
  hidden(str: string): string {
    return createEscape(8) + str + createEscape(28);
  },
  strikethrough(str: string): string {
    return createEscape(9) + str + createEscape(29);
  },
  black(str: string): string {
    return createEscape(30) + str + createEscape(39);
  },
  brightBlack(str: string): string {
    return createEscape(90) + str + createEscape(39);
  },
  red(str: string): string {
    return createEscape(31) + str + createEscape(39);
  },
  brightRed(str: string): string {
    return createEscape(91) + str + createEscape(39);
  },
  green(str: string): string {
    return createEscape(32) + str + createEscape(39);
  },
  brightGreen(str: string): string {
    return createEscape(92) + str + createEscape(39);
  },
  yellow(str: string): string {
    return createEscape(33) + str + createEscape(39);
  },
  brightYellow(str: string): string {
    return createEscape(93) + str + createEscape(39);
  },
  blue(str: string): string {
    return createEscape(34) + str + createEscape(39);
  },
  brightBlue(str: string): string {
    return createEscape(94) + str + createEscape(39);
  },
  magenta(str: string): string {
    return createEscape(35) + str + createEscape(39);
  },
  brightMagenta(str: string): string {
    return createEscape(95) + str + createEscape(39);
  },
  cyan(str: string): string {
    return createEscape(36) + str + createEscape(39);
  },
  brightCyan(str: string): string {
    return createEscape(96) + str + createEscape(39);
  },
  white(str: string): string {
    return createEscape(37) + str + createEscape(39);
  },
  brightWhite(str: string): string {
    return createEscape(97) + str + createEscape(39);
  },
  bgBlack(str: string): string {
    return createEscape(40) + str + createEscape(49);
  },
  bgBrightBlack(str: string): string {
    return createEscape(100) + str + createEscape(49);
  },
  bgRed(str: string): string {
    return createEscape(41) + str + createEscape(49);
  },
  bgBrightRed(str: string): string {
    return createEscape(101) + str + createEscape(49);
  },
  bgGreen(str: string): string {
    return createEscape(42) + str + createEscape(49);
  },
  bgBrightGreen(str: string): string {
    return createEscape(102) + str + createEscape(49);
  },
  bgYellow(str: string): string {
    return createEscape(43) + str + createEscape(49);
  },
  bgBrightYellow(str: string): string {
    return createEscape(103) + str + createEscape(49);
  },
  bgBlue(str: string): string {
    return createEscape(44) + str + createEscape(49);
  },
  bgBrightBlue(str: string): string {
    return createEscape(104) + str + createEscape(49);
  },
  bgMagenta(str: string): string {
    return createEscape(45) + str + createEscape(49);
  },
  bgBrightMagenta(str: string): string {
    return createEscape(105) + str + createEscape(49);
  },
  bgCyan(str: string): string {
    return createEscape(46) + str + createEscape(49);
  },
  bgBrightCyan(str: string): string {
    return createEscape(106) + str + createEscape(49);
  },
  bgWhite(str: string): string {
    return createEscape(47) + str + createEscape(49);
  },
  bgBrightWhite(str: string): string {
    return createEscape(107) + str + createEscape(49);
  },
};

export function stripAnsi(str: string): string {
  return str.replace(regex, '');
}

export function hasAnsi(str: string): boolean {
  return regex.test(str);
}

const DEFAULT_SPACER = ' ';

export function ansiPad(
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

export const ansiEscapes = {
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
