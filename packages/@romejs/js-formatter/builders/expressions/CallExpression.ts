/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, concat} from '../../tokens';
import {callExpression, AnyNode} from '@romejs/js-ast';

export default function CallExpression(builder: Builder, node: AnyNode): Tokens {
    node =
    node.type === 'OptionalCallExpression' || node.type === 'NewExpression'
      ? node
      : callExpression.assert(node);

  const tokens: Tokens = builder.tokenize(node.callee, node);

  if (node.type === 'OptionalCallExpression') {
    tokens.push(operator('?.'));
  }

  if (node.typeArguments) {
    tokens.push(concat(builder.tokenize(node.typeArguments, node)));
  }

  return [
    concat(tokens),
    operator('('),
    ...builder.tokenizeInnerComments(node),
    builder.tokenizeCommaList(node.arguments, node, {
      trailing: true,
    }),
    operator(')'),
  ];
}
