/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowFunctionTypeParam} from '@romejs/js-ast';

export default function FlowFunctionTypeParam(
  builder: Builder,
  node: FlowFunctionTypeParam,
): Token {
  if (node.name) {
    const tokens: Array<Token> = [builder.tokenize(node.name, node)];

    if (node.meta.optional === true) {
      tokens.push('?');
    }

    if (node.meta.typeAnnotation) {
      tokens.push(':', space, builder.tokenize(node.meta.typeAnnotation, node));
    }

    return concat(tokens);
  } else {
    return builder.tokenize(node.meta.typeAnnotation, node);
  }
}
