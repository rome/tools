/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpCharacter, regExpCharacter} from '@romejs/js-ast';

export default function RegExpCharacter(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
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
        generator.append(node.value);
        return;
    }
  }

  switch (node.value) {
    case '\t':
      generator.append('\\t');
      break;

    case '\n':
      generator.append('\\n');
      break;

    case '\r':
      generator.append('\\r');
      break;

    case '\v':
      generator.append('\\v');
      break;

    case '\f':
      generator.append('\\f');
      break;

    case '\b':
      generator.append('\\b');
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
      generator.append('\\' + node.value);
      break;

    default:
      // TODO octals, null, hex
      generator.append(node.value);
  }
}
