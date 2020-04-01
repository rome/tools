/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, classPrivateProperty} from '@romejs/js-ast';
import {Tokens, operator, space} from '@romejs/js-generator/tokens';

export default function ClassPrivateProperty(
  generator: Generator,
  node: AnyNode,
) {
  node = classPrivateProperty.assert(node);

  let tokens: Tokens = [
    ...generator.print(node.meta, node),
    ...generator.print(node.key, node),
    ...generator.printTypeColon(node.typeAnnotation, node),
  ];

  if (node.value) {
    tokens.push(space);
    tokens.push(operator('='));
    tokens.push(space);
    tokens = tokens.concat(generator.print(node.value, node));
  }

  tokens.push(operator(';'));

  return tokens;
}
