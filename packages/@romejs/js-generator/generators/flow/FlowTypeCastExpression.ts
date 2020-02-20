/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowTypeCastExpression,
  flowTypeCastExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTypeCastExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = flowTypeCastExpression.assert(node);

  if (generator.options.typeAnnotations) {
    generator.token('(');
    generator.print(node.expression, node);
    generator.printTypeColon(node.typeAnnotation, node);
    generator.token(')');
  } else {
    generator.print(node.expression, node);
  }
}
