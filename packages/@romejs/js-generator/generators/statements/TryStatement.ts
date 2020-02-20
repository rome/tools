/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {TryStatement, tryStatement, AnyNode} from '@romejs/js-ast';

export default function TryStatement(generator: Generator, node: AnyNode) {
  node = tryStatement.assert(node);
  tryStatement.assert(node);
  generator.word('try');
  generator.space();
  generator.print(node.block, node);
  generator.space();
  generator.print(node.handler, node);

  if (node.finalizer) {
    generator.space();
    generator.word('finally');
    generator.space();
    generator.print(node.finalizer, node);
  }
}
