/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  StringKeywordTypeAnnotation,
  stringKeywordTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';
import StringT from '../../types/StringT';

export default function StringKeywordTypeAnnotation(
  node: AnyNode,
  scope: Scope,
) {
  node = stringKeywordTypeAnnotation.assert(node);
  return new StringT(scope, node);
}
