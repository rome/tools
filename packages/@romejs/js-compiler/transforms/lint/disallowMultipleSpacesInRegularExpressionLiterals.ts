/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpCharacter} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {extractSourceLocationRangeFromNodes} from '@romejs/parser-core';

export default {
  name: 'disallowMultipleSpacesInRegularExpressionLiterals',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'RegExpSubExpression') {
      for (let i = 0; i < node.body.length; i++) {
        const spaceNodes: Array<RegExpCharacter> = [];

        while (true) {
          const item = node.body[i];
          if (item.type === 'RegExpCharacter' && item.value === ' ') {
            spaceNodes.push(item);
            i++;
          } else {
            break;
          }
        }

        if (spaceNodes.length > 1) {
          context.addLocDiagnostic(
            extractSourceLocationRangeFromNodes(spaceNodes),
            {
              category:
                'lint/disallowMultipleSpacesInRegularExpressionLiterals',
              message:
                "Multiple spaces aren't allowed in regular expression literals",
            },
          );
        }
      }
    }

    return node;
  },
};
