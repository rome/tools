/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {AnyNode} from '@romejs/js-ast';

function isEmpty(node: AnyNode): boolean {
  if (node.innerComments !== undefined && node.innerComments.length > 0) {
    return false;
  }

  if (node.type === 'EmptyStatement') {
    return true;
  }

  if (node.type === 'BlockStatement' && node.body.length === 0) {
    return true;
  }

  return false;
}

export default {
  name: 'emptyBlocks',
  enter(path: Path): TransformExitResult {
    const {node, context} = path;

    if (node.type === 'IfStatement') {
      if (isEmpty(node.consequent)) {
        context.addNodeDiagnostic(node.consequent, {
          category: 'lint/emptyBlocks',
          message: 'Empty block',
        });
      }
    }

    return node;
  },
};
