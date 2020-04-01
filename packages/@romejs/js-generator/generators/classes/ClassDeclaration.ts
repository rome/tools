/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator, word} from '../../tokens';
import {ClassDeclaration, classDeclaration, AnyNode} from '@romejs/js-ast';

export default function ClassDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = node.type === 'ClassExpression' ? node : classDeclaration.assert(node);

  let tokens: Tokens = [word('class')];

  if (node.id) {
    tokens = [...tokens, space, ...generator.print(node.id, node)];
  }

  return [
    ...tokens,
    ...generator.print(node.meta, node),
    space,
    operator('{'),
    ...generator.printInnerComments(node),
    ...generator.printInnerComments(node.meta),
    ...generator.printStatementList(node.meta.body, node.meta, true),
    operator('}'),
  ];
}
