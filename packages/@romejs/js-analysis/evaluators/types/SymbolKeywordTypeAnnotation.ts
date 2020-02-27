/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  SymbolKeywordTypeAnnotation,
  symbolKeywordTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function SymbolKeywordTypeAnnotation(node: AnyNode) {
  node = symbolKeywordTypeAnnotation.assert(node);
  throw new Error('unimplemented');
}
