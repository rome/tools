/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  FlowObjectTypeCallProperty,
  flowObjectTypeCallProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeCallProperty(
  node: AnyNode,
  scope: Scope,
) {
  node = flowObjectTypeCallProperty.assert(node);

  return scope.evaluate(node.value);
}
