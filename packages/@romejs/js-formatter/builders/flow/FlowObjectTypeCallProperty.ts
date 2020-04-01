/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {
  FlowObjectTypeCallProperty,
  flowObjectTypeCallProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeCallProperty(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowObjectTypeCallProperty.assert(node);

  if (node.static === true) {
    return [word('static'), space, ...builder.tokenize(node.value, node)];
  } else {
    return builder.tokenize(node.value, node);
  }
}
