/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowClassImplements,
  flowClassImplements,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowClassImplements(node: AnyNode, scope: Scope) {
  node = flowClassImplements.assert(node);
  throw new Error('unimplemented');
}
