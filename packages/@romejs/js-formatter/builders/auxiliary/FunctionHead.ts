/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FunctionHead, functionHead, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {printBindingPatternParams} from '../utils';
import {
  space,
  operator,
  Tokens,
  linkedGroups,
} from '@romejs/js-formatter/tokens';

export default function FunctionHead(builder: Builder, node: AnyNode): Tokens {
  node = functionHead.assert(node);

  const {typeAnnotations} = builder.options;

  let tokens: Tokens = [
    operator('('),
    ...printBindingPatternParams(builder, node, node.params, node.rest),
    operator(')'),
  ];

  if (typeAnnotations) {
    if (node.returnType) {
      tokens = tokens.concat(builder.printTypeColon(node.returnType, node));
    }

    if (node.predicate) {
      if (!node.returnType) {
        tokens.push(operator(':'));
      }
      tokens.push(space);
      tokens = tokens.concat(builder.print(node.predicate, node));
    }
  }

  return [...builder.print(node.typeParameters, node), linkedGroups(tokens)];
}
