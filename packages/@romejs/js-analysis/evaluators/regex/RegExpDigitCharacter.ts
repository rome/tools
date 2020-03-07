/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {AnyNode, RegExpDigitCharacter, regExpDigitCharacter} from '@romejs/js-ast';

export default function RegExpDigitCharacter(node: AnyNode) {
  node = regExpDigitCharacter.assert(node);
  throw new Error('unimplemented');
}
 