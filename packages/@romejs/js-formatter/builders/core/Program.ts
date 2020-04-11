/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {program, AnyNode} from '@romejs/js-ast';
import {Tokens, newline, concat} from '@romejs/js-formatter/tokens';

export default function Program(builder: Builder, node: AnyNode): Tokens {
  node = program.assert(node);

  const tokens: Tokens = builder.tokenizeStatementList(node.directives, node);

  if (node.directives && node.directives.length) {
    tokens.push(newline);
  }

  return [
    concat(tokens),
    concat(builder.tokenizeInnerComments(node)),
    concat(builder.tokenizeStatementList(node.body, node)),
  ];
}
