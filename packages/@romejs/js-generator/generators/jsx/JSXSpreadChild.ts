/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXSpreadChild, jsxSpreadChild, AnyNode} from '@romejs/js-ast';

export default function JSXSpreadChild(generator: Generator, node: AnyNode) {
  node = jsxSpreadChild.assert(node);

  generator.token('{');
  generator.token('...');
  generator.print(node.expression, node);
  generator.token('}');
}
