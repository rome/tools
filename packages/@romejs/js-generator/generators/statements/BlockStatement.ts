/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, newline, indent, operator} from '../../tokens';
import {BlockStatement, blockStatement, AnyNode} from '@romejs/js-ast';

export default function BlockStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = blockStatement.assert(node);

  let tokens: Tokens = [
    operator('{'),
    indent(generator.printInnerComments(node)),
  ];

  const hasDirectives: boolean = Boolean(
    node.directives && node.directives.length > 0,
  );

  if (node.body.length > 0 || hasDirectives) {
    tokens = [
      ...tokens,
      newline,
      indent([
        ...generator.printStatementList(node.directives, node),
        // TODO newline here if hasDirectives
        ...generator.printStatementList(node.body, node),
      ]),
    ];
  }

  return [...tokens, operator('}')];
}
