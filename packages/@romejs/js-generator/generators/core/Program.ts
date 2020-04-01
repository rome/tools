/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Program, program, AnyNode} from '@romejs/js-ast';
import {Tokens, newline} from '@romejs/js-generator/tokens';

export default function Program(generator: Generator, node: AnyNode): Tokens {
  node = program.assert(node);

  const tokens: Tokens = generator.printStatementList(node.directives, node);

  if (node.directives && node.directives.length) {
    tokens.push(newline);
  }

  return [
    ...tokens,
    ...generator.printInnerComments(node),
    ...generator.printStatementList(node.body, node),
  ];
}
