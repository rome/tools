/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  FlowQualifiedTypeIdentifier,
  flowQualifiedTypeIdentifier,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowQualifiedTypeIdentifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowQualifiedTypeIdentifier.assert(node);

  return [
    ...builder.tokenize(node.qualification, node),
    operator('.'),
    ...builder.tokenize(node.id, node),
  ];
}
