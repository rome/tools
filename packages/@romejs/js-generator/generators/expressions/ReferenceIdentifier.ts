/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ReferenceIdentifier,
  referenceIdentifier,
  AnyNode,
} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';

export default function ReferenceIdentifier(
  generator: Generator,
  node: AnyNode,
) {
  node = referenceIdentifier.assert(node);
  Identifier(generator, node);
}
