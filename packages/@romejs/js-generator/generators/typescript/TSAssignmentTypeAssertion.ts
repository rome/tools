/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TSAssignmentTypeAssertion,
  tsAssignmentTypeAssertion,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentTypeAssertion(
  generator: Generator,
  node: AnyNode,
) {
  node = tsAssignmentTypeAssertion.assert(node);
  generator.token('<');
  generator.print(node.typeAnnotation, node);
  generator.token('>');
  generator.space();
  generator.print(node.expression, node);
}
