/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FunctionHead, functionHead, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {printBindingPatternParams} from '../utils';
import {
  space,
  operator,
  Tokens,
  linkedGroups,
} from '@romejs/js-generator/tokens';

export default function FunctionHead(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = functionHead.assert(node);

  const {typeAnnotations} = generator.options;

  let tokens: Tokens = [
    operator('('),
    ...printBindingPatternParams(generator, node, node.params, node.rest),
    operator(')'),
  ];

  if (typeAnnotations) {
    if (node.returnType) {
      tokens = tokens.concat(generator.printTypeColon(node.returnType, node));
    }

    if (node.predicate) {
      if (!node.returnType) {
        tokens.push(operator(':'));
      }
      tokens.push(space);
      tokens = tokens.concat(generator.print(node.predicate, node));
    }
  }

  return [...generator.print(node.typeParameters, node), linkedGroups(tokens)];
}
