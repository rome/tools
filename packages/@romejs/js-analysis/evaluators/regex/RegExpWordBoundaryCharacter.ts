/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  RegExpWordBoundaryCharacter,
  regExpWordBoundaryCharacter,
} from '@romejs/js-ast';

export default function RegExpWordBoundaryCharacter(node: AnyNode) {
  node = regExpWordBoundaryCharacter.assert(node);
  throw new Error('unimplemented');
}
