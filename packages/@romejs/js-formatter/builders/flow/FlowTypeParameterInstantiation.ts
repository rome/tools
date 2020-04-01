/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  FlowTypeParameterInstantiation,
  flowTypeParameterInstantiation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTypeParameterInstantiation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'FlowTypeParameterDeclaration'
    ? node
    : flowTypeParameterInstantiation.assert(node);

  return [
    operator('<'),
    builder.printCommaList(node.params, node),
    operator('>'),
  ];
}
