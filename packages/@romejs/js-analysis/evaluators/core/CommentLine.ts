/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {CommentLine, commentLine, AnyNode} from '@romejs/js-ast';

export default function CommentLine(node: AnyNode, scope: Scope) {
  node = commentLine.assert(node);
  scope;
  throw new Error('unimplemented');
}
