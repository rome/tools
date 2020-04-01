/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, newline, indent, operator} from '../../tokens';
import {BlockStatement, blockStatement, AnyNode} from '@romejs/js-ast';

export default function BlockStatement(builder: Builder, node: AnyNode): Tokens {
  node = blockStatement.assert(node);

  let tokens: Tokens = [
    operator('{'),
    indent(builder.tokenizeInnerComments(node)),
  ];

  const hasDirectives: boolean = Boolean(node.directives &&
      node.directives.length >
      0);

  if (node.body.length > 0 || hasDirectives) {
    tokens = [
      ...tokens,
      newline,
      indent([
        ...builder.tokenizeStatementList(node.directives, node),
        // TODO newline here if hasDirectives
        ...builder.tokenizeStatementList(node.body, node),
      ]),
    ];
  }

  return [...tokens, operator('}')];
}
