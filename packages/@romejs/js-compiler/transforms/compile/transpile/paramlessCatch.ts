/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, bindingIdentifier} from '@romejs/js-ast';

export default {
  name: 'paramlessCatch',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'CatchClause' && node.param === undefined) {
      return {
        ...node,
        param: bindingIdentifier.create({
          name: path.scope.generateUid(),
        }),
      };
    }

    return node;
  },
};
