/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {PrivateName, privateName, AnyNode} from '@romejs/js-ast';

export default function PrivateName(node: AnyNode, scope: Scope) {
  node = privateName.assert(node);
  throw new Error('unimplemented');
}
