/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, classDeclaration} from '@romejs/js-ast';

export default function ClassDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'ClassExpression' ? node : classDeclaration.assert(node);

  const tokens: Tokens = [word('class')];

  if (node.id) {
    tokens.push(space, concat(builder.tokenize(node.id, node)));
  }

  return [
    concat(tokens),
    concat(builder.tokenize(node.meta, node)),
    space,
    operator('{'),
    concat(builder.tokenizeInnerComments(node)),
    concat(builder.tokenizeInnerComments(node.meta)),
    concat(builder.tokenizeStatementList(node.meta.body, node.meta, true)),
    operator('}'),
  ];
}
