/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {AnyNode, classPrivateProperty} from '@romejs/js-ast';
import {Tokens, operator, space} from '@romejs/js-formatter/tokens';

export default function ClassPrivateProperty(builder: Builder, node: AnyNode) {
  node = classPrivateProperty.assert(node);

  let tokens: Tokens = [
    ...builder.print(node.meta, node),
    ...builder.print(node.key, node),
    ...builder.printTypeColon(node.typeAnnotation, node),
  ];

  if (node.value) {
    tokens.push(space);
    tokens.push(operator('='));
    tokens.push(space);
    tokens = tokens.concat(builder.print(node.value, node));
  }

  tokens.push(operator(';'));

  return tokens;
}
