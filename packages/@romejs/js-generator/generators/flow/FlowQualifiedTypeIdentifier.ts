/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {
  FlowQualifiedTypeIdentifier,
  flowQualifiedTypeIdentifier,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowQualifiedTypeIdentifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowQualifiedTypeIdentifier.assert(node);

  return [
    ...generator.print(node.qualification, node),
    operator('.'),
    ...generator.print(node.id, node),
  ];
}
