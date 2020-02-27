/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  EmptyKeywordTypeAnnotation,
  emptyKeywordTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function EmptyKeywordTypeAnnotation(
  node: AnyNode,
  scope: Scope,
) {
  node = emptyKeywordTypeAnnotation.assert(node);
  throw new Error('unimplemented');
}
