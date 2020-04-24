/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, indent, newline, operator} from '../../tokens';
import {AnyNode, blockStatement} from '@romejs/js-ast';

export default function BlockStatement(builder: Builder, node: AnyNode): Tokens {
  node = blockStatement.assert(node);

  const tokens: Tokens = [
    operator('{'),
    indent(builder.tokenizeInnerComments(node)),
  ];

  const hasDirectives: boolean = Boolean(node.directives &&
      node.directives.length >
      0);

  if (node.body.length > 0 || hasDirectives) {
    tokens.push(newline, indent([
      concat(builder.tokenizeStatementList(node.directives, node)),
      // TODO newline here if hasDirectives
      concat(builder.tokenizeStatementList(node.body, node)),
    ]));
  }

  return [concat(tokens), operator('}')];
}
