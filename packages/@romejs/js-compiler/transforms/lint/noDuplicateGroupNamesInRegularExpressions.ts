/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpGroupCapture} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noDuplicateGroupNamesInRegularExpressions',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'RegExpSubExpression') {
      const groupUsage = new Map<string, Array<RegExpGroupCapture>>();

      for (const bodyItem of node.body) {
        if (bodyItem.type === 'RegExpGroupCapture') {
          const groupName = bodyItem.name;

          if (groupName !== undefined) {
            let usages = groupUsage.get(groupName);

            if (usages === undefined) {
              usages = [];
              groupUsage.set(groupName, usages);
            }
            usages.push(bodyItem);
          }
        }
      }

      for (const [name, usages] of groupUsage) {
        if (usages.length === 1) {
          continue;
        }

        const firstUsage = usages[0];

        context.addNodeDiagnostic(firstUsage, {
          description: descriptions.LINT.DUPLICATE_REGEX_GROUP_NAME(
            name,
            usages.slice(1).map((node) => node.loc),
          ),
        });
      }
    }
    return node;
  },
};
