/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noExplicitAny',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'AnyKeywordTypeAnnotation') {
      context.addNodeDiagnostic(node, {
        category: 'lint/noExplicitAny',
        message: 'Unexpected any. Specify a different type.',
      });
    }

    return node;
  },
};
