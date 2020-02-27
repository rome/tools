/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {DoExpression, doExpression, AnyNode} from '@romejs/js-ast';

export default function DoExpression(generator: Generator, node: AnyNode) {
  node = doExpression.assert(node);
  doExpression.assert(node);
  generator.word('do');
  generator.space();
  generator.print(node.body, node);
}
