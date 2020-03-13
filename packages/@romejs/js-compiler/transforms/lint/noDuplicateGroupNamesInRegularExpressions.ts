/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {SourceLocation} from '@romejs/parser-core';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';

export default {
  name: 'noDuplicateGroupNamesInRegularExpressions',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'RegExpSubExpression') {
      const uniqueGroupNames = new Map<string, undefined | SourceLocation>();

      for (const regExpBodyItem of node.body) {
        if (regExpBodyItem.type === 'RegExpGroupCapture') {
          const advice: PartialDiagnosticAdvice = [];
          const group = regExpBodyItem;
          const groupName = group.name;

          if (groupName !== undefined) {
            const originalLoc = uniqueGroupNames.get(groupName);

            if (originalLoc !== undefined) {
              advice.push({
                type: 'log',
                category: 'info',
                message: 'Originally defined here',
              });

              advice.push({
                type: 'frame',
                ...originalLoc,
              });

              context.addNodeDiagnostic(group, {
                category: 'lint/noDuplicateGroupNamesInRegularExpressions',
                message: `Duplicate group name <emphasis>${groupName}</emphasis> in regular expression`,
                advice,
              });
            }
            uniqueGroupNames.set(groupName, group.loc);
          }
        }
      }
    }
    return node;
  },
};
