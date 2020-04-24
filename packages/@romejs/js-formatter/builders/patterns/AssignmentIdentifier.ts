/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  AssignmentIdentifier,
  assignmentIdentifier,
} from '@romejs/js-ast';
import Identifier from '../auxiliary/Identifier';

export default function AssignmentIdentifier(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = assignmentIdentifier.assert(node);
  return Identifier(builder, node);
}
