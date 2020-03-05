/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {getCompletionRecords} from '@romejs/js-ast-utils';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'unReachableCode',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'BlockStatement') {
      const records = getCompletionRecords(node);
      for (const record of records) {
        if (
          record.type === 'COMPLETION' &&
          node.body[node.body.length - 1].type !== 'ReturnStatement'
        ) {
          context.addNodeDiagnostic(node.body[node.body.length - 1], {
            category: 'lint/unRecheableCode',
            message: 'Unrecheable code',
          });
        }
      }
    }
    return node;
  },
};
