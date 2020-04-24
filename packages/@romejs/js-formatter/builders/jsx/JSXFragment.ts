/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, indent, operator} from '../../tokens';
import {AnyNode, jsxFragment} from '@romejs/js-ast';

export default function JSXFragment(builder: Builder, node: AnyNode): Tokens {
  node = jsxFragment.assert(node);

  return [
    operator('<>'),
    indent(node.children.map((child) => concat(builder.tokenize(child, node)))),
    operator('</>'),
  ];
}
