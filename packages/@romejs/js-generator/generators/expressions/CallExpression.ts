/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {CallExpression, callExpression, AnyNode} from '@romejs/js-ast';

export default function CallExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
    node =
    node.type === 'OptionalCallExpression' || node.type === 'NewExpression'
      ? node
      : callExpression.assert(node);

  const tokens: Tokens = [
    ...generator.print(node.callee, node),
    ...generator.print(node.typeArguments, node),
  ];

  if (node.type === 'OptionalCallExpression') {
    tokens.push(operator('?'));
  }

  return [
    ...tokens,
    operator('('),
    generator.printCommaList(node.arguments, node, {
      trailing: true,
    }),
    operator(')'),
  ];
}
