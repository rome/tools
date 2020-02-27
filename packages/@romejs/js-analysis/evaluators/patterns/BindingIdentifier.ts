/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {BindingIdentifier, bindingIdentifier, AnyNode} from '@romejs/js-ast';

export default function BindingIdentifier(node: AnyNode, scope: Scope) {
  node = bindingIdentifier.assert(node);
  throw new Error('unimplemented');
}
