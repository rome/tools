/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXFragment, jsxFragment, AnyNode} from '@romejs/js-ast';

export default function JSXFragment(generator: Generator, node: AnyNode) {
  node = jsxFragment.assert(node);
  jsxFragment.assert(node);
  generator.token('<>');
  generator.indent();

  for (const child of node.children) {
    generator.print(child, node);
  }

  generator.dedent();
  generator.token('</>');
}
