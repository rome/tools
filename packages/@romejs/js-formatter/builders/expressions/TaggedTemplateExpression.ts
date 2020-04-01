/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  TaggedTemplateExpression,
  taggedTemplateExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TaggedTemplateExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = taggedTemplateExpression.assert(node);

  return [...builder.print(node.tag, node), ...builder.print(node.quasi, node)];
}
