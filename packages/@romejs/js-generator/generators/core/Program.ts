/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Program, program, AnyNode} from '@romejs/js-ast';

export default function Program(generator: Generator, node: AnyNode) {
  node = program.assert(node);

  generator.printInnerComments(node, false);
  generator.printStatementList(node.directives, node);

  if (node.directives && node.directives.length) {
    generator.forceNewline();
  }

  generator.printStatementList(node.body, node);
}
