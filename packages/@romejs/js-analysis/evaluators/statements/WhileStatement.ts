/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {WhileStatement, whileStatement, AnyNode} from '@romejs/js-ast';

export default function WhileStatement(node: AnyNode, scope: Scope) {
  node = whileStatement.assert(node);
  scope;
  throw new Error('unimplemented');
}
