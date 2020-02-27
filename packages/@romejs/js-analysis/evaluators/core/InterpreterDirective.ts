/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  InterpreterDirective,
  interpreterDirective,
  AnyNode,
} from '@romejs/js-ast';

export default function InterpreterDirective(node: AnyNode, scope: Scope) {
  node = interpreterDirective.assert(node);
  throw new Error('unimplemented');
}
