/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {WithStatement, withStatement, AnyNode} from '@romejs/js-ast';

export default function WithStatement(generator: Generator, node: AnyNode) {
  node = withStatement.assert(node);

  generator.word('with');
  generator.space();
  generator.token('(');
  generator.print(node.object, node);
  generator.token(')');
  generator.printBlock(node);
}
