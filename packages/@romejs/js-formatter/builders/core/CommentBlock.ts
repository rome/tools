/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {AnyNode, CommentBlock, commentBlock} from '@romejs/js-ast';

export default function CommentBlock(builder: Builder, node: AnyNode): Tokens {
  node = commentBlock.assert(node);
  commentBlock.assert(node);
  throw new Error('unimplemented');
}
