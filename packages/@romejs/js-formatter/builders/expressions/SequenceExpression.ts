/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {AnyNode, sequenceExpression} from '@romejs/js-ast';

export default function SequenceExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = sequenceExpression.assert(node);
  return [builder.tokenizeCommaList(node.expressions, node)];
}
