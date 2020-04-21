/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BaseTokens, ValueToken, SimpleToken} from '@romejs/parser-core';

export type Tokens = BaseTokens & {
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
  name: MarkupTagName;
  attributes: TagAttributes;
  children: Children;
};

export type ChildNode = TextNode | TagNode;

export type Children = Array<ChildNode>;

export type MarkupTagName =
  | 'pad'
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
  | 'error'
  | 'success'
  | 'warn'
  | 'info'
  | 'highlight';
