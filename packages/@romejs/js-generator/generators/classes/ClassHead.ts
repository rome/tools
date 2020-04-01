/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space} from '../../tokens';
import {ClassHead, classHead, AnyNode} from '@romejs/js-ast';

export default function ClassHead(generator: Generator, node: AnyNode): Tokens {
  node = classHead.assert(node);

  let tokens: Tokens = generator.print(node.typeParameters, node);

  if (node.superClass) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      ...generator.print(node.superClass, node),
      ...generator.print(node.superTypeParameters, node),
    ];
  }

  if (
    node.implements !== undefined &&
    node.implements.length > 0 &&
    generator.options.typeAnnotations
  ) {
    tokens = [
      ...tokens,
      space,
      word('implements'),
      space,
      generator.printCommaList(node.implements, node),
    ];
  }

  return tokens;
}
