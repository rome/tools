/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {TypeAliasTypeAnnotation} from '@romejs/js-ast';

export default function TypeAliasTypeAnnotation(
  builder: Builder,
  node: TypeAliasTypeAnnotation,
): Token {
  return concat([
    'type',
    space,
    builder.tokenize(node.id, node),
    builder.tokenize(node.typeParameters, node),
    space,
    '=',
    space,
    builder.tokenize(node.right, node),
    ';',
  ]);
}
