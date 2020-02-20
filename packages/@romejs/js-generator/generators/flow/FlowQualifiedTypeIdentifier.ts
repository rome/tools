/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowQualifiedTypeIdentifier,
  flowQualifiedTypeIdentifier,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowQualifiedTypeIdentifier(
  generator: Generator,
  node: AnyNode,
) {
  node = flowQualifiedTypeIdentifier.assert(node);

  generator.print(node.qualification, node);
  generator.token('.');
  generator.print(node.id, node);
}
