/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {SwitchCase, switchCase, AnyNode} from '@romejs/js-ast';

export default function SwitchCase(generator: Generator, node: AnyNode) {
  node = switchCase.assert(node);

  if (node.test) {
    generator.word('case');
    generator.space();
    generator.print(node.test, node);
    generator.token(':');
  } else {
    generator.word('default');
    generator.token(':');
  }

  generator.forceNewline();

  if (node.consequent.length) {
    generator.printStatementList(node.consequent, node, {indent: true});
  }
}
