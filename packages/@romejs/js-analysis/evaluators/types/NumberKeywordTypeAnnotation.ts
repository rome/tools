/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  NumberKeywordTypeAnnotation,
  numberKeywordTypeAnnotation,
} from '@romejs/js-ast';
import NumericT from '../../types/NumericT';

export default function NumberKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
  node = numberKeywordTypeAnnotation.assert(node);
  return new NumericT(scope, node);
}
