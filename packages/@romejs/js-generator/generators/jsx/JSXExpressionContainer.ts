/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  JSXExpressionContainer,
  jsxExpressionContainer,
  AnyNode,
} from '@romejs/js-ast';

export default function JSXExpressionContainer(
  generator: Generator,
  node: AnyNode,
) {
  node = jsxExpressionContainer.assert(node);

  generator.token('{');
  generator.print(node.expression, node);
  generator.token('}');
}
