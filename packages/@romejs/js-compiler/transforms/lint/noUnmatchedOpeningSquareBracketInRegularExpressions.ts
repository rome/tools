/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpCharacter} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

export default {
  name: 'noUnmatchedOpeningSquareBracketInRegularExpressions',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    let lastBracketCharacter: Partial<RegExpCharacter> = {};
    let hasOpeningBracket: boolean = false;

    if (node.type === 'RegExpSubExpression') {
      for (const item of node.body) {
        if (item.type === 'RegExpCharacter') {
          if (item.value === '[') {
            hasOpeningBracket = true;
          }

          lastBracketCharacter = item;
        }
      }

      if (hasOpeningBracket && lastBracketCharacter.value === '[') {
        context.addNodeDiagnostic(node, {
          category: 'lint/noUnmatchedOpeningSquareBracketInRegularExpressions',
          message:
            'Unmatched opening square brackets in regular expressions are not allowed',
          start: lastBracketCharacter.loc?.start,
          end: lastBracketCharacter.loc?.end,
        });
      }
    }

    return node;
  },
};
