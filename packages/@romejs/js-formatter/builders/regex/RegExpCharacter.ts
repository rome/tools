/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, verbatim} from '../../tokens';
import {AnyNode, RegExpCharacter, regExpCharacter} from '@romejs/js-ast';
import {escapeString} from '@romejs/string-escape';

export default function RegExpCharacter(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = regExpCharacter.assert(node);

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
        return [verbatim(node.value)];

      case '-':
        return [verbatim('\\-')];
    }
  }

  switch (node.value) {
    case '\t':
      return [verbatim('\\t')];

    case '\n':
      return [verbatim('\\n')];

    case '\r':
      return [verbatim('\\r')];

    case '\x0b':
      return [verbatim('\\v')];

    case '\f':
      return [verbatim('\\f')];

    case '\b':
      return [verbatim('\\b')];

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
      return [verbatim(`\\${node.value}`)];

    default:
      return [
        verbatim(escapeString(node.value, {
          json: true,
          unicodeOnly: true,
        })),
      ];
  }
}
