/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FlowTypeParameter, flowTypeParameter, AnyNode} from '@romejs/js-ast';

export default function FlowTypeParameter(generator: Generator, node: AnyNode) {
  node = flowTypeParameter.assert(node);

  generator.print(node.variance, node);
  generator.word(node.name);

  if (node.bound) {
    generator.print(node.bound, node);
  }

  if (node.default) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.default, node);
  }
}
