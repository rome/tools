/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {SpreadElement, spreadElement, AnyNode} from '@romejs/js-ast';
import {operator, Tokens} from '@romejs/js-formatter/tokens';

export default function SpreadElement(builder: Builder, node: AnyNode): Tokens {
  node = spreadElement.assert(node);

  return [operator('...'), ...builder.print(node.argument, node)];
}
