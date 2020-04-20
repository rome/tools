/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {AnyNode, classPrivateProperty} from '@romejs/js-ast';
import {Tokens, operator, space, concat} from '@romejs/js-formatter/tokens';

export default function ClassPrivateProperty(builder: Builder, node: AnyNode) {
  node = classPrivateProperty.assert(node);

  const tokens: Tokens = [
    concat(builder.tokenize(node.meta, node)),
    concat(builder.tokenize(node.key, node)),
    concat(builder.tokenizeTypeColon(node.typeAnnotation, node)),
  ];

  if (node.value) {
    tokens.push(space);
    tokens.push(operator('='));
    tokens.push(space);
    tokens.push(concat(builder.tokenize(node.value, node)));
  }

  tokens.push(operator(';'));

  return tokens;
}
