/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpCharSet, regExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(node: AnyNode) {
  node = regExpCharSet.assert(node);
  throw new Error('unimplemented');
}
