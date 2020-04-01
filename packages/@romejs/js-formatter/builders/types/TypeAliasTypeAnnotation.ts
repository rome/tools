/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, operator} from '../../tokens';
import {
  TypeAliasTypeAnnotation,
  typeAliasTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function TypeAliasTypeAnnotation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = typeAliasTypeAnnotation.assert(node);

  return [
    word('type'),
    space,
    ...builder.print(node.id, node),
    ...builder.print(node.typeParameters, node),
    space,
    operator('='),
    space,
    ...builder.print(node.right, node),
    operator(';'),
  ];
}
