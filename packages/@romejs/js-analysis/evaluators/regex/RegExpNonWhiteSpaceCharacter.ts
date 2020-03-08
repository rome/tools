/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  RegExpNonWhiteSpaceCharacter,
  regExpNonWhiteSpaceCharacter,
} from '@romejs/js-ast';

export default function RegExpNonWhiteSpaceCharacter(node: AnyNode) {
  node = regExpNonWhiteSpaceCharacter.assert(node);
  throw new Error('unimplemented');
}
