/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXAttribute, jsxAttribute, AnyNode} from '@romejs/js-ast';

export default function JSXAttribute(generator: Generator, node: AnyNode) {
  node = jsxAttribute.assert(node);
  jsxAttribute.assert(node);
  generator.print(node.name, node);
  if (node.value) {
    generator.token('=');
    generator.print(node.value, node);
  }
}
