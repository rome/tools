/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXMemberExpression, jsxMemberExpression, AnyNode} from '@romejs/js-ast';

export default function JSXMemberExpression(generator: Generator, node: AnyNode) {
  node = jsxMemberExpression.assert(node);

  generator.print(node.object, node);
  generator.token('.');
  generator.print(node.property, node);
}
