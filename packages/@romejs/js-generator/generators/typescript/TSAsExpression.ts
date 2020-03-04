/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSAsExpression, tsAsExpression, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSAsExpression(generator: Generator, node: AnyNode) {
  node = tsAsExpression.assert(node);

  generator.print(node.expression);

  if (generator.options.typeAnnotations) {
    generator.word('as');
    generator.print(node.typeAnnotation, node);
  }
}
