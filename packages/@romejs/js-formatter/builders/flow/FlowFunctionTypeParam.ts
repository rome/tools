/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space} from '../../tokens';
import {
  FlowFunctionTypeParam,
  flowFunctionTypeParam,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeParam(builder: Builder, node: AnyNode) {
  node = flowFunctionTypeParam.assert(node);

  if (node.name) {
    const tokens: Tokens = builder.tokenize(node.name, node);

    if (node.meta.optional === true) {
      tokens.push(operator('?'));
    }

    return [
      ...tokens,
      operator(':'),
      space,
      ...builder.tokenize(node.meta.typeAnnotation, node),
    ];
  } else {
    return builder.tokenize(node.meta.typeAnnotation, node);
  }
}
