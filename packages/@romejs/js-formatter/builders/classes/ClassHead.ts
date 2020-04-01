/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space} from '../../tokens';
import {ClassHead, classHead, AnyNode} from '@romejs/js-ast';

export default function ClassHead(builder: Builder, node: AnyNode): Tokens {
  node = classHead.assert(node);

  let tokens: Tokens = builder.tokenize(node.typeParameters, node);

  if (node.superClass) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      ...builder.tokenize(node.superClass, node),
      ...builder.tokenize(node.superTypeParameters, node),
    ];
  }

  if (node.implements !== undefined && node.implements.length > 0 &&
      builder.options.typeAnnotations) {
    tokens = [
      ...tokens,
      space,
      word('implements'),
      space,
      builder.tokenizeCommaList(node.implements, node),
    ];
  }

  return tokens;
}
