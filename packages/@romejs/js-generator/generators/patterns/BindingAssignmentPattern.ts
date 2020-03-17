/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  BindingAssignmentPattern,
  bindingAssignmentPattern,
  AnyNode,
} from '@romejs/js-ast';

export default function BindingAssignmentPattern(
  generator: Generator,
  node: AnyNode,
) {
  node = bindingAssignmentPattern.assert(node);

  generator.print(node.left, node);
  generator.space();
  generator.token('=');
  generator.space();
  generator.print(node.right, node);
}
