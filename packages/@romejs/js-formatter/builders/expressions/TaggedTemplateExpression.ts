/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat} from '../../tokens';
import {AnyNode, taggedTemplateExpression} from '@romejs/js-ast';

export default function TaggedTemplateExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = taggedTemplateExpression.assert(node);

  return [
    concat(builder.tokenize(node.tag, node)),
    concat(builder.tokenize(node.quasi, node)),
  ];
}
