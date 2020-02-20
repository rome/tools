/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {DebuggerStatement, debuggerStatement, AnyNode} from '@romejs/js-ast';

export default function DebuggerStatement(node: AnyNode, scope: Scope) {
  node = debuggerStatement.assert(node);
  throw new Error('unimplemented');
}
