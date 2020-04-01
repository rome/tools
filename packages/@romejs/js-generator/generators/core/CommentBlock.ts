/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {CommentBlock, commentBlock, AnyNode} from '@romejs/js-ast';

export default function CommentBlock(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = commentBlock.assert(node);
  commentBlock.assert(node);
  throw new Error('unimplemented');
}
