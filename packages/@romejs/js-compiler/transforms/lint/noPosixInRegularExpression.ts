/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpCharSet} from '@romejs/js-ast';
import {Path, Context} from '@romejs/js-compiler';

function checkRegEx(node: RegExpCharSet, context: Context): RegExpCharSet {
  node.body.forEach((currNode, i) => {
    const nextNode = node.body[i + 1];
    const lastNode = node.body[node.body.length - 1];
    if (
      currNode.type === 'RegExpCharacter' &&
      currNode.value === '[' &&
      nextNode.type === 'RegExpCharacter' &&
      (nextNode.value === ':' || nextNode.value === '.') &&
      lastNode.type === 'RegExpCharacter' &&
      lastNode.value === nextNode.value
    ) {
      context.addNodeDiagnostic(currNode, {
        fixable: false,
        category: 'lint/noPosixInRegularExpression',
        message:
          'POSIX Character Classes and Collating Sequences are not supported in ECMAscript Regular Expressions',
      });
    }
  });

  return node;
}

export default {
  name: 'noPosixInRegularExpression',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'RegExpCharSet') {
      return checkRegEx(node, context);
    }

    return node;
  },
};
