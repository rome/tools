/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowTypeofTypeAnnotation,
  flowTypeofTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTypeofTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = flowTypeofTypeAnnotation.assert(node);

  generator.word('typeof');
  generator.space();
  generator.print(node.argument, node);
}
