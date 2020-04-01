/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space, operator} from '../../tokens';
import {
  TypeAliasTypeAnnotation,
  typeAliasTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function TypeAliasTypeAnnotation(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = typeAliasTypeAnnotation.assert(node);

  return [
    word('type'),
    space,
    ...generator.print(node.id, node),
    ...generator.print(node.typeParameters, node),
    space,
    operator('='),
    space,
    ...generator.print(node.right, node),
    operator(';'),
  ];
}
