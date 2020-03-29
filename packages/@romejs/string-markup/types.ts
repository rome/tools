/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BaseTokens, ValueToken, SimpleToken} from '@romejs/parser-core';

export type Tokens =
  & BaseTokens
  & {
    Text: ValueToken<'Text', string>;
    Slash: SimpleToken<'Slash'>;
    Less: SimpleToken<'Less'>;
    Equals: SimpleToken<'Equals'>;
    Greater: SimpleToken<'Greater'>;
    Word: ValueToken<'Word', string>;
    String: ValueToken<'String', string>;
  };

//
export type TextNode = {
  type: 'Text';
  value: string;
};

export type TagAttributes = Map<string, string>;

export type TagNode = {
  type: 'Tag';
  name: TagName;
  attributes: TagAttributes;
  children: Children;
};

export type ChildNode = TextNode | TagNode;

export type Children = Array<ChildNode>;

export type TagName =
  | 'grammarNumber'
  | 'command'
  | 'inverse'
  | 'dim'
  | 'emphasis'
  | 'number'
  | 'hyperlink'
  | 'filelink'
  | 'duration'
  | 'filesize'
  | 'italic'
  | 'underline'
  | 'strike'
  | 'black'
  | 'brightBlack'
  | 'red'
  | 'brightRed'
  | 'green'
  | 'brightGreen'
  | 'yellow'
  | 'brightYellow'
  | 'blue'
  | 'brightBlue'
  | 'magenta'
  | 'brightMagenta'
  | 'cyan'
  | 'brightCyan'
  | 'white'
  | 'brightWhite'
  | 'bgBlack'
  | 'bgBrightBlack'
  | 'bgRed'
  | 'bgBrightRed'
  | 'bgGreen'
  | 'bgBrightGreen'
  | 'bgYellow'
  | 'bgBrightYellow'
  | 'bgBlue'
  | 'bgBrightBlue'
  | 'bgMagenta'
  | 'bgBrightMagenta'
  | 'bgCyan'
  | 'bgBrightCyan'
  | 'bgWhite'
  | 'bgBrightWhite';
