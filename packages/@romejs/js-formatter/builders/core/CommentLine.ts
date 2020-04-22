/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {AnyNode, CommentLine, commentLine} from '@romejs/js-ast';

export default function CommentLine(builder: Builder, node: AnyNode): Tokens {
  node = commentLine.assert(node);
  commentLine.assert(node);
  throw new Error('unimplemented');
}
