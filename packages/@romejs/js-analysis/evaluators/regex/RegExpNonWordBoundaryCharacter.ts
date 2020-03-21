/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  RegExpNonWordBoundaryCharacter,
  regExpNonWordBoundaryCharacter,
} from '@romejs/js-ast';

export default function RegExpNonWordBoundaryCharacter(node: AnyNode) {
  node = regExpNonWordBoundaryCharacter.assert(node);
  throw new Error('unimplemented');
}
