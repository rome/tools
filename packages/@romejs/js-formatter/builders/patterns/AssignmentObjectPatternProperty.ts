/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {AssignmentObjectPatternProperty} from '@romejs/js-ast';
import ObjectProperty from '../objects/ObjectProperty';

export default function AssignmentObjectPatternProperty(
  builder: Builder,
  node: AssignmentObjectPatternProperty,
): Token {
  return ObjectProperty(builder, node);
}
