/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {AnyNode, RegExpGroupNonCapture, regExpGroupNonCapture} from '@romejs/js-ast';

export default function RegExpGroupNonCapture(node: AnyNode) {
  node = regExpGroupNonCapture.assert(node);
  throw new Error('unimplemented');
}
 