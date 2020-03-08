/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  RegExpWhiteSpaceCharacter,
  regExpWhiteSpaceCharacter,
} from '@romejs/js-ast';

export default function RegExpWhiteSpaceCharacter(node: AnyNode) {
  node = regExpWhiteSpaceCharacter.assert(node);
  throw new Error('unimplemented');
}
