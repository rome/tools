/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {DoWhileStatement, doWhileStatement, AnyNode} from '@romejs/js-ast';

export default function DoWhileStatement(generator: Generator, node: AnyNode) {
  node = doWhileStatement.assert(node);

  generator.word('do');
  generator.space();
  generator.print(node.body, node);
  generator.space();
  generator.word('while');
  generator.space();
  generator.token('(');
  generator.print(node.test, node);
  generator.token(')');
  generator.semicolon();
}
