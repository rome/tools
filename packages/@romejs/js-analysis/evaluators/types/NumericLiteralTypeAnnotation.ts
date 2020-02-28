/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  AnyNode,
  NumericLiteralTypeAnnotation,
  numericLiteralTypeAnnotation,
} from '@romejs/js-ast';

export default function NumericLiteralTypeAnnotation(
  node: AnyNode,
  scope: Scope,
) {
  node = numericLiteralTypeAnnotation.assert(node);
  scope;
  throw new Error('unimplemented');
}
