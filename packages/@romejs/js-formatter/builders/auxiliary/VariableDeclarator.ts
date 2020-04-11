/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {variableDeclarator, AnyNode} from '@romejs/js-ast';
import {operator, space, concat} from '@romejs/js-formatter/tokens';

export default function VariableDeclarator(builder: Builder, node: AnyNode) {
  node = variableDeclarator.assert(node);

  if (node.init) {
    return [
      concat(builder.tokenize(node.id, node)),
      space,
      operator('='),
      space,
      concat(builder.tokenize(node.init, node)),
    ];
  } else {
    return builder.tokenize(node.id, node);
  }
}
