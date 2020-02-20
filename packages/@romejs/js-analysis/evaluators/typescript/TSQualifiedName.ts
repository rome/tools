/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {TSQualifiedName, tsQualifiedName, AnyNode} from '@romejs/js-ast';

export default function TSQualifiedName(node: AnyNode, scope: Scope) {
  node = tsQualifiedName.assert(node);
  throw new Error('unimplemented');
}
