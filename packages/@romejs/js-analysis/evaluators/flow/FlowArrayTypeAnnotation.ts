/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  FlowArrayTypeAnnotation,
  flowArrayTypeAnnotation,
} from '@romejs/js-ast';

export default function FlowArrayTypeAnnotation(node: AnyNode, scope: Scope) {
  node = flowArrayTypeAnnotation.assert(node);
  scope;
  throw new Error('unimplemented');
}
