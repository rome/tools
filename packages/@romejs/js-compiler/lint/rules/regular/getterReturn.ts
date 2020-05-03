/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {getCompletionRecords} from '@romejs/js-ast-utils';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'getterReturn',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      (node.type === 'ClassMethod' || node.type === 'ObjectMethod') &&
      node.kind === 'get'
    ) {
      for (const record of getCompletionRecords(node.body)) {
        if (record.type === 'INVALID') {
          path.context.addNodeDiagnostic(
            record.node,
            descriptions.LINT.GETTER_RETURN(record.description),
          );
        }
      }
    }

    return node;
  },
};
