/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {ImportSpecifier, importSpecifier, AnyNode} from '@romejs/js-ast';

export default function ImportSpecifier(node: AnyNode, scope: Scope) {
  node = importSpecifier.assert(node);
  throw new Error('unimplemented');
}
