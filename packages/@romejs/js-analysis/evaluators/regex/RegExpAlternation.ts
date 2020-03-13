/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpAlternation, regExpAlternation} from '@romejs/js-ast';

export default function RegExpAlternation(node: AnyNode) {
  node = regExpAlternation.assert(node);
  throw new Error('unimplemented');
}
