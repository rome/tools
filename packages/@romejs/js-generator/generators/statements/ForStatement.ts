/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ForStatement, forStatement, AnyNode} from '@romejs/js-ast';

export default function ForStatement(generator: Generator, node: AnyNode) {
  node = forStatement.assert(node);
  forStatement.assert(node);
  generator.word('for');
  generator.space();
  generator.token('(');

  generator.multiline(
    node,
    (multiline, node) => {
      generator.inForStatementInitCounter++;
      generator.print(node.init, node);
      generator.inForStatementInitCounter--;
      generator.token(';');

      if (node.test) {
        generator.spaceOrNewline(multiline);
        generator.print(node.test, node);
      }
      generator.token(';');

      if (node.update) {
        generator.spaceOrNewline(multiline);
        generator.print(node.update, node);
      }

      generator.token(')');
      generator.printBlock(node);
    },
    {conditions: ['more-than-one-line']},
  );
}
