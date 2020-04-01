/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space} from '../../tokens';
import {DoExpression, doExpression, AnyNode} from '@romejs/js-ast';

export default function DoExpression(builder: Builder, node: AnyNode): Tokens {
  node = doExpression.assert(node);

  return [word('do'), space, ...builder.print(node.body, node)];
}
