/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {WithStatement, withStatement, AnyNode} from '@romejs/js-ast';

export default function WithStatement(node: AnyNode, scope: Scope) {
  node = withStatement.assert(node);
  scope;
  throw new Error('unimplemented');
}
