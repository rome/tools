/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Children, TagAttributes, MarkupTagName, TagNode} from './types';
import {parseMarkup} from './parse';
import {
  humanizeNumber,
  humanizeTime,
  humanizeFileSize,
} from '@romejs/string-utils';
import {formatAnsi, ansiPad} from './ansi';
import {AbsoluteFilePath, createUnknownFilePath} from '@romejs/path';
import {escapeMarkup} from './escape';

export type MarkupFormatFilenameNormalizer = (filename: string) => string;

export type MarkupFormatFilenameHumanizer = (
  filename: string,
) => undefined | string;

export type MarkupFormatOptions = {
  normalizeFilename?: MarkupFormatFilenameNormalizer;
  humanizeFilename?: MarkupFormatFilenameHumanizer;
  cwd?: AbsoluteFilePath;
};

const EMPTY_ATTRIBUTES: Map<string, string> = new Map();

type FormatReduceOptions = {
  ancestry: Array<TagNode>;
  formatText: (value: string, tags: Array<TagNode>) => string;
  formatTag?: (
    name: MarkupTagName,
    attributes: TagAttributes,
    value: string,
  ) => string;
};

function formatReduceFromInput(input: string, opts: FormatReduceOptions): string {
  return formatReduceFromChildren(parseMarkup(input), opts);
}

// Ignore nested bare tags eg. <emphasis><emphasis>foo</emphasis></emphasis>
function shouldIgnoreTag(
  tagName: MarkupTagName,
  opts: FormatReduceOptions,
): boolean {
  for (const tag of opts.ancestry) {
    if (tag.attributes.size === 0 && tag.name === tagName) {
      return true;
    }
  }
  return false;
}

function formatReduceFromChildren(
  children: Children,
  opts: FormatReduceOptions,
): string {
  const {formatTag, formatText} = opts;

  let buff = '';
  for (const child of children) {
    if (child.type === 'Text') {
      buff += formatText(child.value, opts.ancestry);
    } else if (child.type === 'Tag') {
      const {attributes} = child;

      let emphasis = attributes.get('emphasis') === 'true' && !shouldIgnoreTag(
        'emphasis',
        opts,
      );
      attributes.delete('emphasis');

      let dim = attributes.get('dim') === 'true' &&
        !shouldIgnoreTag('dim', opts);
      attributes.delete('dim');

      const applyTags: Array<TagNode> = [];
      if (attributes.size > 0 || attributes.size === 0 && !shouldIgnoreTag(
          child.name,
          opts,
        )) {
        applyTags.push(child);
      }
      if (emphasis) {
        applyTags.push({
          type: 'Tag',
          name: 'emphasis',
          attributes: EMPTY_ATTRIBUTES,
          children: [],
        });
      }
      if (dim) {
        applyTags.push({
          type: 'Tag',
          name: 'dim',
          attributes: EMPTY_ATTRIBUTES,
          children: [],
        });
      }

      let res = formatReduceFromChildren(child.children, {
        ...opts,
        ancestry: [...opts.ancestry, ...applyTags],
      });

      if (formatTag !== undefined) {
        for (const tag of applyTags) {
          res = formatTag(tag.name, tag.attributes, res);
        }
      }

      buff += res;
    } else {
      throw new Error('Unknown child node type');
    }
  }
  return buff;
}

function formatFileLink(
  attributes: TagAttributes,
  value: string,
  opts: MarkupFormatOptions,
): {
  text: string;
  filename: string;
} {
  let text = value;

  // Normalize filename
  let filename = attributes.get('target') || '';
  if (opts.normalizeFilename !== undefined) {
    filename = opts.normalizeFilename(filename);
  }

  // Default text to a humanized version of the filename
  if (text === '') {
    text = humanizeMarkupFilename(filename, opts);

    const line = attributes.get('line');
    if (line !== undefined) {
      text += `:${line}`;

      const column = attributes.get('column');
      // Ignore a 0 column and just target the line
      if (column !== undefined && column !== '0') {
        text += `:${column}`;
      }
    }
  }

  return {text, filename};
}

function formatApprox(attributes: TagAttributes, value: string) {
  if (attributes.get('approx') === 'true') {
    return `~${value}`;
  } else {
    return value;
  }
}

function formatGrammarNumber(attributes: TagAttributes, value: string) {
  const num = Number(value);

  const none = attributes.get('none');
  if (none !== undefined && num === 0) {
    return none;
  }

  const singular = attributes.get('singular');
  if (singular !== undefined && num === 1) {
    return singular;
  }

  const plural = attributes.get('plural');
  if (plural !== undefined) {
    return plural;
  }

  return '';
}

function formatNumber(attributes: TagAttributes, value: string) {
  const num = Number(value);
  const human = humanizeNumber(num);
  const humanWithApprox = formatApprox(attributes, human);
  return humanWithApprox;
}

function formatPad(attributes: TagAttributes, value: string) {
  const left = attributes.get('dir') !== 'right';
  const count = Number(attributes.get('count') || 0);
  const char = attributes.get('char');
  return ansiPad(left ? 'left' : 'right', value, count, char);
}

export function markupToPlainText(
  input: string,
  opts: MarkupFormatOptions = {},
): string {
  return formatReduceFromInput(input, {
    ancestry: [],
    formatText: (text) => {
      return text;
    },
    formatTag: (tag, attributes, value) => {
      switch (tag) {
        case 'filelink':
          return formatFileLink(attributes, value, opts).text;

        case 'number':
          return formatNumber(attributes, value);

        case 'grammarNumber':
          return formatGrammarNumber(attributes, value);

        case 'duration':
          return formatApprox(attributes, humanizeTime(Number(value), true));

        case 'filesize':
          return humanizeFileSize(Number(value));

        case 'pad':
          return formatPad(attributes, value);

        case 'command':
          return `\`${value}\``;

        case 'italic':
          return `_${value}_`;

        default:
          return value;
      }
    },
  });
}

export function normalizeMarkup(
  input: string,
  opts: MarkupFormatOptions = {},
): string {
  return formatReduceFromInput(input, {
    ancestry: [],
    formatText: (text) => {
      return escapeMarkup(text);
    },
    formatTag: (tag, attributes, value) => {
      // Normalize filename of <filelink target>
      if (tag === 'filelink') {
        const {text, filename} = formatFileLink(attributes, value, opts);
        attributes.set('target', filename);
        value = text;
      }

      let attrStr = Array.from(attributes, ([key, value]) => {
        return `${key}="${value.replace(/"/g, '\\"')}"`;
      }).join(' ');

      let open = `<${tag}`;
      if (attrStr !== '') {
        open += ` ${attrStr}`;
      }

      if (value === '') {
        return `${open} />`;
      } else {
        return `${open}>${value}</${tag}>`;
      }
    },
  });
}

export function markupToAnsi(
  input: string,
  opts: MarkupFormatOptions = {},
): string {
  return formatReduceFromInput(input, {
    ancestry: [],
    formatText: (value, tags) => {
      // Format tags in reverse
      for (let i = tags.length - 1; i >= 0; i--) {
        const tag = tags[i];
        value = ansiFormatTag(tag, value, opts);
      }

      return formatAnsi.reset(value);
    },
  });
}

function ansiFormatTag(
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
      return formatPad(attributes, value);

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

    case 'black':
      return formatAnsi.black(value);

    case 'brightBlack':
      return formatAnsi.brightBlack(value);

    case 'red':
      return formatAnsi.red(value);

    case 'brightRed':
      return formatAnsi.brightRed(value);

    case 'green':
      return formatAnsi.green(value);

    case 'brightGreen':
      return formatAnsi.brightGreen(value);

    case 'yellow':
      return formatAnsi.yellow(value);

    case 'brightYellow':
      return formatAnsi.brightYellow(value);

    case 'blue':
      return formatAnsi.blue(value);

    case 'brightBlue':
      return formatAnsi.brightBlue(value);

    case 'magenta':
      return formatAnsi.magenta(value);

    case 'brightMagenta':
      return formatAnsi.brightMagenta(value);

    case 'cyan':
      return formatAnsi.cyan(value);

    case 'brightCyan':
      return formatAnsi.brightCyan(value);

    case 'white':
      return formatAnsi.white(value);

    case 'brightWhite':
      return formatAnsi.brightWhite(value);

    case 'bgBlack':
      return formatAnsi.bgBlack(value);

    case 'bgBrightBlack':
      return formatAnsi.bgBrightBlack(value);

    case 'bgRed':
      return formatAnsi.bgRed(value);

    case 'bgBrightRed':
      return formatAnsi.bgBrightRed(value);

    case 'bgGreen':
      return formatAnsi.bgGreen(value);

    case 'bgBrightGreen':
      return formatAnsi.bgBrightGreen(value);

    case 'bgYellow':
      return formatAnsi.bgYellow(value);

    case 'bgBrightYellow':
      return formatAnsi.bgBrightYellow(value);

    case 'bgBlue':
      return formatAnsi.bgBlue(value);

    case 'bgBrightBlue':
      return formatAnsi.bgBrightBlue(value);

    case 'bgMagenta':
      return formatAnsi.bgMagenta(value);

    case 'bgBrightMagenta':
      return formatAnsi.bgBrightMagenta(value);

    case 'bgCyan':
      return formatAnsi.bgCyan(value);

    case 'bgBrightCyan':
      return formatAnsi.bgBrightCyan(value);

    case 'bgWhite':
      return formatAnsi.bgWhite(value);

    case 'bgBrightWhite':
      return formatAnsi.bgBrightWhite(value);

    case 'command':
      return '`' + formatAnsi.italic(value) + '`';
  }
}

export function humanizeMarkupFilename(
  filename: string,
  opts: MarkupFormatOptions = {},
): string {
  if (opts.humanizeFilename !== undefined) {
    const override = opts.humanizeFilename(filename);
    if (override !== undefined) {
      return override;
    }
  }

  return createUnknownFilePath(filename).format(opts.cwd);
}
