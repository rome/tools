/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word} from '../../tokens';
import {classPropertyMeta, AnyNode} from '@romejs/js-ast';

export default function ClassPropertyMeta(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = classPropertyMeta.assert(node);

  const tokens: Tokens = [];

  if (!builder.options.typeAnnotations) {
    if (node.accessibility) {
      tokens.push(word(node.accessibility));
    }

    if (node.readonly) {
      tokens.push(word('readonly'));
    }

    if (node.abstract) {
      tokens.push(word('abstract'));
    }
  }

  if (node.static) {
    tokens.push(word('static'));
  }

  return tokens;
}
