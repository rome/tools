/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {AnyNode, RegExpCharacter} from '@romejs/js-ast';
import {escapeString} from '@romejs/string-escape';

export default function RegExpCharacter(
  builder: Builder,
  node: RegExpCharacter,
  parent: AnyNode,
): Token {
  const isInCharSet = parent.type === 'RegExpCharSet';
  if (isInCharSet) {
    switch (node.value) {
      case '$':
      case '^':
      case '.':
      case '?':
      case '{':
      case '}':
      case '+':
      case '*':
      case '[':
      case ']':
      case '(':
      case ')':
      case '|':
        return node.value;

      case '-':
        return '\\-';
    }
  }

  switch (node.value) {
    case '\t':
      return '\\t';

    case '\n':
      return '\\n';

    case '\r':
      return '\\r';

    case '\x0b':
      return '\\v';

    case '\f':
      return '\\f';

    case '\b':
      return '\\b';

    case '/':
    case '\\':
    case '$':
    case '^':
    case '.':
    case '?':
    case '{':
    case '}':
    case '+':
    case '*':
    case '[':
    case ']':
    case '(':
    case ')':
    case '|':
      return `\\${node.value}`;

    default:
      return escapeString(node.value, {json: true, unicodeOnly: true});
  }
}
