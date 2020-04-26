/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  BooleanKeywordTypeAnnotation,
  booleanKeywordTypeAnnotation,
} from '@romejs/js-ast';
import BooleanT from '../../types/BooleanT';

export default function BooleanKeywordTypeAnnotation(
  node: AnyNode,
  scope: Scope,
) {
  node = booleanKeywordTypeAnnotation.assert(node);
  return new BooleanT(scope, node);
}
