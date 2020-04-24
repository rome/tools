/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  MarkupFormatOptions,
  formatApprox,
  formatFileLink,
  formatGrammarNumber,
  formatNumber,
  formatPad,
  formatReduceFromInput,
} from './format';
import {humanizeFileSize, humanizeTime} from '@romejs/string-utils';
import {formatAnsi} from './ansi';
import {MarkupTagName, TagAttributes, TagNode} from './types';

export function markupToAnsi(
  input: string,
  opts: MarkupFormatOptions = {},
): string {
  return formatReduceFromInput(input, {
    ancestry: [],
    formatTag: ansiFormatTag,
    formatText: (value, tags) => {
      // Format tags in reverse
      for (let i = tags.length - 1; i >= 0; i--) {
        const tag = tags[i];
        value = ansiFormatText(tag, value, opts);
      }

      return formatAnsi.reset(value);
    },
  });
}

function ansiFormatTag(
  tagName: MarkupTagName,
  attributes: TagAttributes,
  value: string,
): string {
  switch (tagName) {
    case 'pad':
      return formatPad(attributes, value);

    case 'command':
      return '`' + value + '`';

    default:
      return value;
  }
}

function ansiFormatText(
  {name: tagName, attributes}: TagNode,
  value: string,
  opts: MarkupFormatOptions,
): string {
  switch (tagName) {
    case 'hyperlink': {
      let text = value;
      let hyperlink = attributes.get('target');

      if (hyperlink === undefined) {
        hyperlink = text;
      }

      if (text === '') {
        text = hyperlink;
      }

      return formatAnsi.hyperlink(text, hyperlink);
    }

    case 'pad':
      return value;

    case 'filelink': {
      const {text, filename} = formatFileLink(attributes, value, opts);
      return formatAnsi.hyperlink(text, `file://${filename}`);
    }

    case 'inverse':
      return formatAnsi.inverse(` ${value} `);

    case 'emphasis':
      return formatAnsi.bold(value);

    case 'dim':
      return formatAnsi.dim(value);

    case 'filesize':
      return humanizeFileSize(Number(value));

    case 'duration':
      return formatApprox(attributes, humanizeTime(Number(value), true));

    case 'number':
      return formatNumber(attributes, value);

    case 'grammarNumber':
      return formatGrammarNumber(attributes, value);

    case 'italic':
      return formatAnsi.italic(value);

    case 'underline':
      return formatAnsi.underline(value);

    case 'strike':
      return formatAnsi.strikethrough(value);

    case 'error':
      return formatAnsi.red(value);

    case 'success':
      return formatAnsi.green(value);

    case 'warn':
      return formatAnsi.yellow(value);

    case 'info':
      return formatAnsi.blue(value);

    case 'command':
      return formatAnsi.italic(value);

    case 'highlight': {
      const index = Math.min(0, Number(attributes.get('i')) || 0);
      const showLegend = attributes.get('legend') === 'true';
      const fn = ansiHighlightFactories[index % ansiHighlightFactories.length];
      let formatted = fn(value);
      if (showLegend) {
        formatted += formatAnsi.dim(`[${String(index + 1)}]`);
      }
      return formatted;
    }

    case 'color':
      return formatAnsiBackground(attributes.get('bg'), formatAnsiForeground(
        attributes.get('fg'),
        value,
      ));
  }
}

// TODO fill this
const ansiHighlightFactories: Array<(str: string) => string> = [
  formatAnsi.magenta,
  formatAnsi.cyan,
];

function formatAnsiBackground(bg: undefined | string, text: string): string {
  if (bg === undefined) {
    return text;
  }

  switch (bg) {
    case 'black':
      return formatAnsi.bgBlack(text);

    case 'brightBlack':
      return formatAnsi.bgBrightBlack(text);

    case 'red':
      return formatAnsi.bgRed(text);

    case 'brightRed':
      return formatAnsi.bgBrightRed(text);

    case 'green':
      return formatAnsi.bgGreen(text);

    case 'brightGreen':
      return formatAnsi.bgBrightGreen(text);

    case 'yellow':
      return formatAnsi.bgYellow(text);

    case 'brightYellow':
      return formatAnsi.bgBrightYellow(text);

    case 'blue':
      return formatAnsi.bgBlue(text);

    case 'brightBlue':
      return formatAnsi.bgBrightBlue(text);

    case 'magenta':
      return formatAnsi.bgMagenta(text);

    case 'brightMagenta':
      return formatAnsi.bgBrightMagenta(text);

    case 'cyan':
      return formatAnsi.bgCyan(text);

    case 'brightCyan':
      return formatAnsi.bgBrightCyan(text);

    case 'white':
      return formatAnsi.bgWhite(text);

    case 'brightWhite':
      return formatAnsi.bgBrightWhite(text);

    default:
      return text;
  }
}

function formatAnsiForeground(fg: undefined | string, text: string): string {
  if (fg === undefined) {
    return text;
  }

  switch (fg) {
    case 'black':
      return formatAnsi.black(text);

    case 'brightBlack':
      return formatAnsi.brightBlack(text);

    case 'red':
      return formatAnsi.red(text);

    case 'brightRed':
      return formatAnsi.brightRed(text);

    case 'green':
      return formatAnsi.green(text);

    case 'brightGreen':
      return formatAnsi.brightGreen(text);

    case 'yellow':
      return formatAnsi.yellow(text);

    case 'brightYellow':
      return formatAnsi.brightYellow(text);

    case 'blue':
      return formatAnsi.blue(text);

    case 'brightBlue':
      return formatAnsi.brightBlue(text);

    case 'magenta':
      return formatAnsi.magenta(text);

    case 'brightMagenta':
      return formatAnsi.brightMagenta(text);

    case 'cyan':
      return formatAnsi.cyan(text);

    case 'brightCyan':
      return formatAnsi.brightCyan(text);

    case 'white':
      return formatAnsi.white(text);

    case 'brightWhite':
      return formatAnsi.brightWhite(text);

    default:
      return text;
  }
}
