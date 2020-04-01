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
      break;

    case '\n':
      return [verbatim('\\n')];
      break;

    case '\r':
      return [verbatim('\\r')];
      break;

    case '\x0b':
      return [verbatim('\\v')];
      break;

    case '\f':
      return [verbatim('\\f')];
      break;

    case '\b':
      return [verbatim('\\b')];
      break;

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
