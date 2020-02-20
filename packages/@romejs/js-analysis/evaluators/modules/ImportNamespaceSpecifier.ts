/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  ImportNamespaceSpecifier,
  importNamespaceSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ImportNamespaceSpecifier(node: AnyNode, scope: Scope) {
  node = importNamespaceSpecifier.assert(node);
  throw new Error('unimplemented');
}
