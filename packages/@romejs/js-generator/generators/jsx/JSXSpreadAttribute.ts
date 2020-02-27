/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXSpreadAttribute, jsxSpreadAttribute, AnyNode} from '@romejs/js-ast';

export default function JSXSpreadAttribute(
  generator: Generator,
  node: AnyNode,
) {
  node = jsxSpreadAttribute.assert(node);

  generator.token('{');
  generator.token('...');
  generator.print(node.argument, node);
  generator.token('}');
}
