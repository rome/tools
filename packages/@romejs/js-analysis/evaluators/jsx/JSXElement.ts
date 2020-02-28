/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {JSXElement, jsxElement, AnyNode} from '@romejs/js-ast';

export default function JSXElement(node: AnyNode, scope: Scope) {
  node = jsxElement.assert(node);
  scope;
  throw new Error('unimplemented');
}
