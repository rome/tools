/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  AssignmentIdentifier,
  assignmentIdentifier,
  AnyNode,
} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';

export default function AssignmentIdentifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = assignmentIdentifier.assert(node);
  return Identifier(generator, node);
}
