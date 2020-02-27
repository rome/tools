/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FlowVariance, flowVariance, AnyNode} from '@romejs/js-ast';

export default function FlowVariance(generator: Generator, node: AnyNode) {
  node = flowVariance.assert(node);
  flowVariance.assert(node);
  if (node.kind === 'plus') {
    generator.token('+');
  } else if (node.kind === 'minus') {
    generator.token('-');
  }
}
