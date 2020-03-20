/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpGroupCapture} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';

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

        const duplicateAdvice: PartialDiagnosticAdvice = usages
          .slice(1)
          .map(node => {
            if (node.loc === undefined) {
              return {
                type: 'log',
                category: 'warn',
                message: 'Unable to find location',
              };
            } else {
              return {
                type: 'frame',
                ...node.loc,
              };
            }
          });

        context.addNodeDiagnostic(firstUsage, {
          category: 'lint/noDuplicateGroupNamesInRegularExpressions',
          message: `Duplicate group name <emphasis>${name}</emphasis> in regular expression`,
          advice: [
            {
              type: 'log',
              category: 'info',
              message: 'Defined again here',
            },
            ...duplicateAdvice,
          ],
        });
      }
    }
    return node;
  },
};
