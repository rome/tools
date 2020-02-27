/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  isStrictReservedWord,
  isStrictBindReservedWord,
  isES2015ReservedWord,
  isKeyword,
  isIdentifierStart,
  isIdentifierChar,
  getFullCharCodeAt,
} from '@romejs/js-parser-utils';

/**
 * This performs a basic check to see if a string is a valid identifier name.
 *
 * Note that sometimes this may return false positives. For example, there are some
 * identifiers that are valid as references but not as bindings.
 *
 * So whatever you decide to do with this check, ensure that it's not causing any
 * unintentional semantics.
 */
export default function isValidIdentifierName(name: string): boolean {
  if (name.length === 0) {
    return false;
  }

  if (isStrictReservedWord(name, true)) {
    return false;
  }

  if (isStrictBindReservedWord(name, true)) {
    return false;
  }

  if (isES2015ReservedWord(name)) {
    return false;
  }

  if (isKeyword(name)) {
    return false;
  }

  if (isIdentifierStart(getFullCharCodeAt(name, 0)) === false) {
    return false;
  }

  let i = 1;
  while (i < name.length) {
    const code = getFullCharCodeAt(name, i);
    if (isIdentifierChar(code)) {
      i += code <= 0xffff ? 1 : 2;
    } else {
      return false;
    }
  }

  return true;
}
