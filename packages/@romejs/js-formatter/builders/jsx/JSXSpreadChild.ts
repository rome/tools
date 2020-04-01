/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {JSXSpreadChild, jsxSpreadChild, AnyNode} from '@romejs/js-ast';

export default function JSXSpreadChild(builder: Builder, node: AnyNode): Tokens {
  node = jsxSpreadChild.assert(node);

  return [
    operator('{'),
    operator('...'),
    ...builder.print(node.expression, node),
    operator('}'),
  ];
}
