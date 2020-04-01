/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word} from '../../tokens';
import {
  FlowObjectTypeCallProperty,
  flowObjectTypeCallProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeCallProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowObjectTypeCallProperty.assert(node);

  if (node.static === true) {
    return [word('static'), space, ...generator.print(node.value, node)];
  } else {
    return generator.print(node.value, node);
  }
}
