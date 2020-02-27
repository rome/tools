/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {SwitchStatement, switchStatement, AnyNode} from '@romejs/js-ast';

export default function SwitchStatement(generator: Generator, node: AnyNode) {
  node = switchStatement.assert(node);

  generator.word('switch');
  generator.space();
  generator.token('(');
  generator.print(node.discriminant, node);
  generator.token(')');
  generator.space();
  generator.token('{');

  generator.printStatementList(node.cases, node, {
    indent: true,
  });

  generator.token('}');
}
