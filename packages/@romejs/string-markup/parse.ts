/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ParserOptions,
  createParser,
  isAlpha,
  isEscaped,
  TokenValues,
} from '@romejs/parser-core';
import {
  Tokens,
  TagAttributes,
  TagNode,
  ChildNode,
  Children,
  TagName,
} from './types';
import {inc, Number0, add, get0} from '@romejs/ob1';

const globalAttributes: Array<string> = ['emphasis', 'dim'];

const tags: Map<string, Array<string>> = new Map();
tags.set('emphasis', []);
tags.set('number', ['approx']);
tags.set('hyperlink', ['target']);
tags.set('filelink', ['target', 'column', 'line']);
tags.set('inverse', []);
tags.set('dim', []);
tags.set('filesize', []);
tags.set('duration', ['approx']);
tags.set('italic', []);
tags.set('underline', []);
tags.set('strike', []);
tags.set('black', []);
tags.set('brightBlack', []);
tags.set('red', []);
tags.set('brightRed', []);
tags.set('green', []);
tags.set('brightGreen', []);
tags.set('yellow', []);
tags.set('brightYellow', []);
tags.set('blue', []);
tags.set('brightBlue', []);
tags.set('magenta', []);
tags.set('brightMagenta', []);
tags.set('cyan', []);
tags.set('brightCyan', []);
tags.set('white', []);
tags.set('brightWhite', []);
tags.set('bgBlack', []);
tags.set('bgBrightBlack', []);
tags.set('bgRed', []);
tags.set('bgBrightRed', []);
tags.set('bgGreen', []);
tags.set('bgBrightGreen', []);
tags.set('bgYellow', []);
tags.set('bgBrightYellow', []);
tags.set('bgBlue', []);
tags.set('bgBrightBlue', []);
tags.set('bgMagenta', []);
tags.set('bgBrightMagenta', []);
tags.set('bgCyan', []);
tags.set('bgBrightCyan', []);
tags.set('bgWhite', []);
tags.set('bgBrightWhite', []);
tags.set('command', []);

//
function isStringValueChar(char: string, index: Number0, input: string): boolean {
  if (char === '"' && !isEscaped(index, input)) {
    return false;
  }

  return true;
}

function isTextChar(char: string, index: Number0, input: string): boolean {
  return !isTagChar(index, input);
}

export function isTagChar(index: Number0, input: string): boolean {
  const i = get0(index);
  return input[i] === '<' && !isEscaped(index, input) &&
    (isAlpha(input[i + 1]) || input[i + 1] === '/');
}

type State = {inTagHead: boolean};

type StringMarkupParserOptions = ParserOptions;

const createStringMarkupParser = createParser((ParserCore) =>
  class StringMarkupParser extends ParserCore<Tokens, State> {
    constructor(opts: StringMarkupParserOptions) {
      super(opts, 'parse/stringMarkup', {inTagHead: false});
    }

    tokenizeWithState(
      index: Number0,
      input: string,
      state: State,
    ):
        | undefined
        | {
          token: TokenValues<Tokens>;
          state: State;
        } {
      const escaped = isEscaped(index, input);
      const char = input[get0(index)];

      if (!escaped && state.inTagHead) {
        if (char === ' ') {
          return this.lookahead(inc(index));
        }

        if (char === '=') {
          return {
            state,
            token: this.finishToken('Equals'),
          };
        }

        if (char === '/') {
          return {
            state,
            token: this.finishToken('Slash'),
          };
        }

        if (isAlpha(char)) {
          const [value, end] = this.readInputFrom(index, isAlpha);
          return {
            state,
            token: this.finishValueToken('Word', value, end),
          };
        }

        if (char === '"') {
          const [value, stringValueEnd, unclosed] = this.readInputFrom(
            inc(index),
            isStringValueChar,
          );

          if (unclosed) {
            throw this.unexpected({
              message: 'Unclosed string',
              start: this.getPositionFromIndex(stringValueEnd),
            });
          }

          const end = add(stringValueEnd, 1);
          return {
            state,
            token: this.finishValueToken('String', value, end),
          };
        }

        if (char === '>') {
          return {
            state: {
              inTagHead: false,
            },
            token: this.finishToken('Greater'),
          };
        }
      }

      if (isTagChar(index, input)) {
        return {
          state: {
            inTagHead: true,
          },
          token: this.finishToken('Less'),
        };
      }

      // Keep eating text until we hit a <
      const [value, end] = this.readInputFrom(index, isTextChar);
      return {
        state,
        token: {
          type: 'Text',
          value: normalizeTextValue(value),
          start: index,
          end,
        },
      };
    }

    atTagEnd(): boolean {
      return this.matchToken('Less') && this.lookahead().token.type === 'Slash';
    }

    parseTag(): TagNode {
      const nameToken = this.expectToken('Word');
      const rawName = nameToken.value;

      const allowedAttributes = tags.get(rawName);
      if (allowedAttributes === undefined) {
        throw this.unexpected({
          message: `Unknown tag name <emphasis>${rawName}</emphasis>`,
          start: this.getPositionFromIndex(nameToken.start),
        });
      }

      // rome-suppress lint/noExplicitAny
      const tagName: TagName = (rawName as any);
      const attributes: TagAttributes = new Map();
      const children: Children = [];
      let selfClosing = false;

      // Parse attributes
      while (!this.matchToken('EOF') && !this.matchToken('Greater')) {
        const keyToken = this.getToken();

        let key;
        if (keyToken.type === 'Word') {
          key = keyToken.value;

          if (!allowedAttributes.includes(key) && !globalAttributes.includes(key)) {
            throw this.unexpected({
              message: `${key} is not a valid attribute name for <${tagName}>`,
            });
          }

          this.nextToken();

          // Shorthand properties
          if (this.matchToken('Word') || this.matchToken('Slash') ||
          this.matchToken('Greater')) {
            attributes.set(key, 'true');
            continue;
          }

          this.expectToken('Equals');

          const valueToken = this.expectToken('String');
          if (valueToken.type !== 'String') {
            throw new Error('Expected String');
          }
          const value = valueToken.value;

          attributes.set(key, value);
        } else if (keyToken.type === 'Slash') {
          this.nextToken();
          selfClosing = true;
        } else {
          throw this.unexpected({
            message: 'Expected attribute name',
          });
        }
      }

      this.expectToken('Greater');

      // Verify closing tag
      if (!selfClosing) {
        while ( // Build children
        !this.matchToken('EOF') && !this.atTagEnd()) {
          children.push(this.parseChild());
        }

        if (this.matchToken('EOF')) {
          throw this.unexpected({
            message: `Unclosed ${tagName} tag`,
          });
        } else {
          this.expectToken('Less');
          this.expectToken('Slash');

          const name = this.getToken();
          if (name.type === 'Word') {
            if (name.value !== tagName) {
              throw this.unexpected({
                message: `Expected to close ${tagName} but found ${name.value}`,
              });
            }

            this.nextToken();
          } else {
            throw this.unexpected({
              message: 'Expected closing tag name',
            });
          }

          this.expectToken('Greater');
        }
      }

      return {
        type: 'Tag',
        attributes,
        name: tagName,
        children,
      };
    }

    parseChild(): ChildNode {
      const token = this.getToken();
      this.nextToken();

      if (token.type === 'Text') {
        return {
          type: 'Text',
          value: token.value,
        };
      } else if (token.type === 'Less') {
        return this.parseTag();
      } else {
        throw this.unexpected({
          message: 'Unknown child start',
        });
      }
    }

    parse(): Children {
      const children: Children = [];
      while (!this.matchToken('EOF')) {
        children.push(this.parseChild());
      }
      return children;
    }
  }
);

export function parseMarkup(input: string) {
  try {
    return createStringMarkupParser({input}).parse();
  } catch (err) {
    throw err;
  }
}

function normalizeTextValue(str: string): string {
  return str.replace(/\\<([a-zA-Z\/])/g, '<$1');
}
