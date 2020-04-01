/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, indent, operator, flatten} from '../../tokens';
import {JSXFragment, jsxFragment, AnyNode} from '@romejs/js-ast';

export default function JSXFragment(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = jsxFragment.assert(node);

  return [
    operator('<>'),
    indent(flatten(node.children.map(child => generator.print(child, node)))),
    operator('</>'),
  ];
}
