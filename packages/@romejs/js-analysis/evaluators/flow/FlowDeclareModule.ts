/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {FlowDeclareModule, flowDeclareModule, AnyNode} from '@romejs/js-ast';

export default function FlowDeclareModule(node: AnyNode, scope: Scope) {
  node = flowDeclareModule.assert(node);
  scope;
  throw new Error('unimplemented');
}
