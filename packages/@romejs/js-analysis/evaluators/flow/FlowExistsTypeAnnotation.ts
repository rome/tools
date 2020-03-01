/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowExistsTypeAnnotation,
  flowExistsTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowExistsTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowExistsTypeAnnotation.assert(node);
  scope;
  throw new Error('unimplemented');
}
