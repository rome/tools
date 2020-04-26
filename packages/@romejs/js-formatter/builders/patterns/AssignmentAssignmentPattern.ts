/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AssignmentAssignmentPattern} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token} from '../../tokens';
import BindingAssignmentPattern from './BindingAssignmentPattern';

export default function AssignmentAssignmentPattern(
  builder: Builder,
  node: AssignmentAssignmentPattern,
): Token {
  return BindingAssignmentPattern(builder, node);
}
