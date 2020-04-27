/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Children, MarkupTagName, TagAttributes, TagNode} from './types';
import {parseMarkup} from './parse';
import {
  humanizeFileSize,
  humanizeNumber,
  humanizeTime,
} from '@romejs/string-utils';
import {ansiPad} from './ansi';
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

export function formatReduceFromInput(
  input: string,
  opts: FormatReduceOptions,
): string {
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

  // Sometimes we'll populate the inner text of a tag with no children
  if (children.length === 0) {
    return formatText('', opts.ancestry);
  }

  let buff = '';
  for (const child of children) {
    if (child.type === 'Text') {
      buff += formatText(child.value, opts.ancestry);
    } else if (child.type === 'Tag') {
      // Clone it since we'll be deleting attributes
      const attributes = new Map(child.attributes.entries());

      let emphasis =
        attributes.get('emphasis') === 'true' &&
        !shouldIgnoreTag('emphasis', opts);
      attributes.delete('emphasis');

      let dim =
        attributes.get('dim') === 'true' && !shouldIgnoreTag('dim', opts);
      attributes.delete('dim');

      const applyTags: Array<TagNode> = [];
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
      if (
        attributes.size > 0 ||
        (attributes.size === 0 && !shouldIgnoreTag(child.name, opts))
      ) {
        applyTags.push(child);
      }

      let res = formatReduceFromChildren(
        child.children,
        {
          ...opts,
          ancestry: [...opts.ancestry, ...applyTags],
        },
      );

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

export function formatFileLink(
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

export function formatApprox(attributes: TagAttributes, value: string) {
  if (attributes.get('approx') === 'true') {
    return `~${value}`;
  } else {
    return value;
  }
}

export function formatGrammarNumber(attributes: TagAttributes, value: string) {
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

export function formatNumber(attributes: TagAttributes, value: string) {
  const num = Number(value);
  const human = humanizeNumber(num);
  const humanWithApprox = formatApprox(attributes, human);
  return humanWithApprox;
}

export function formatPad(attributes: TagAttributes, value: string) {
  const left = attributes.get('dir') !== 'right';
  const count = Number(attributes.get('count') || 0);
  const char = attributes.get('char');
  const padded = ansiPad(left ? 'left' : 'right', value, count, char);
  return padded;
}

export function markupToPlainText(
  input: string,
  opts: MarkupFormatOptions = {},
): string {
  return formatReduceFromInput(
    input,
    {
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
    },
  );
}

export type NormalizeMarkupOptions = MarkupFormatOptions & {
  stripPositions?: boolean;
};

export function normalizeMarkup(
  input: string,
  opts: NormalizeMarkupOptions = {},
): string {
  return formatReduceFromInput(
    input,
    {
      ancestry: [],
      formatText: (text) => {
        return escapeMarkup(text);
      },
      formatTag: (tag, attributes, value) => {
        switch (tag) {
          case // Normalize filename of <filelink target>
          'filelink': {
            // Clone
            attributes = new Map([...attributes]);
            const {text, filename} = formatFileLink(attributes, value, opts);
            attributes.set('target', filename);
            if (opts.stripPositions) {
              attributes.delete('line');
              attributes.delete('column');
            }
            value = text;
            break;
          }

          // We don't technically need to normalize this but it's one less tag to have to support
          // if other tools need to consume it
          case 'grammarNumber':
            return formatGrammarNumber(attributes, value);
        }

        let attrStr = Array.from(
          attributes,
          ([key, value]) => {
            if (value === 'true') {
              return key;
            } else {
              const escapedValue = escapeMarkup(value);
              return `${key}="${escapedValue}"`;
            }
          },
        ).join(' ');

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
    },
  );
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
