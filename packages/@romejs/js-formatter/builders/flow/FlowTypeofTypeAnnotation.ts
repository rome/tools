/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {
  AnyNode,
  FlowTypeofTypeAnnotation,
  flowTypeofTypeAnnotation,
} from '@romejs/js-ast';

export default function FlowTypeofTypeAnnotation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowTypeofTypeAnnotation.assert(node);

  return [word('typeof'), space, ...builder.tokenize(node.argument, node)];
}
