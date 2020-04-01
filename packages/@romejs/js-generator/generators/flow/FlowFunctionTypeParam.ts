/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space} from '../../tokens';
import {
  FlowFunctionTypeParam,
  flowFunctionTypeParam,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeParam(
  generator: Generator,
  node: AnyNode,
) {
  node = flowFunctionTypeParam.assert(node);

  if (node.name) {
    const tokens: Tokens = generator.print(node.name, node);

    if (node.meta.optional === true) {
      tokens.push(operator('?'));
    }

    return [
      ...tokens,
      operator(':'),
      space,
      ...generator.print(node.meta.typeAnnotation, node),
    ];
  } else {
    return generator.print(node.meta.typeAnnotation, node);
  }
}
