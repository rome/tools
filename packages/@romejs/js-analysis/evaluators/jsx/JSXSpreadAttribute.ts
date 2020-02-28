/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {JSXSpreadAttribute, jsxSpreadAttribute, AnyNode} from '@romejs/js-ast';

export default function JSXSpreadAttribute(node: AnyNode, scope: Scope) {
  node = jsxSpreadAttribute.assert(node);
  scope;
  throw new Error('unimplemented');
}
