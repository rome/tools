/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noDuplicateGroupNamesInRegularExpressions',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'RegExpSubExpression') {
      const uniqueGroupNames = new Set();

      for (const regExpBodyItem of node.body) {
        if (regExpBodyItem.type === 'RegExpGroupCapture') {
          const group = regExpBodyItem;
          const groupName = group.name;
          if (groupName !== undefined) {
            if (uniqueGroupNames.has(groupName)) {
              context.addNodeDiagnostic(group, {
                category: 'lint/noDuplicateGroupNamesInRegularExpressions',
                message: `Duplicate group name <emphasis>${groupName}</emphasis> in regular expression`,
              });
            }
            uniqueGroupNames.add(groupName);
          }
        }
      }
    }

    return node;
  },
};
