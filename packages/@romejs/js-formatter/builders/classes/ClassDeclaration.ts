/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, operator, word} from '../../tokens';
import {ClassDeclaration, classDeclaration, AnyNode} from '@romejs/js-ast';

export default function ClassDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'ClassExpression' ? node : classDeclaration.assert(node);

  let tokens: Tokens = [word('class')];

  if (node.id) {
    tokens = [...tokens, space, ...builder.tokenize(node.id, node)];
  }

  return [
    ...tokens,
    ...builder.tokenize(node.meta, node),
    space,
    operator('{'),
    ...builder.tokenizeInnerComments(node),
    ...builder.tokenizeInnerComments(node.meta),
    ...builder.tokenizeStatementList(node.meta.body, node.meta, true),
    operator('}'),
  ];
}
