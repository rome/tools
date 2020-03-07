/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

function isValidRegExpPattern(pattern: string) {
  try {
    new RegExp(pattern);
  } catch (err) {
    return false;
  }
  return true;
}

function isValidRegExpFlags(flags: string) {
  try {
    new RegExp('', flags);
  } catch (err) {
    return false;
  }
  return true;
}

export default {
  name: 'noInvalidRegExp',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (
      (node.type === 'NewExpression' || node.type === 'CallExpression') &&
      node.callee.type === 'ReferenceIdentifier' &&
      node.callee.name === 'RegExp'
    ) {
      const [first, second] = node.arguments;

      // Test Pattern
      if (first !== undefined && first.type === 'StringLiteral') {
        const pattern = first.value;
        if (isValidRegExpPattern(pattern) === false) {
          context.addNodeDiagnostic(node.arguments[0], {
            category: 'lint/noInvalidRegExp',
            message:
              // TODO: Add more informative error message once regex parser has been implemented
              `Invalid pattern supplied to RegExp constructor ${pattern}`,
          });
        }
      }

      // Test Flags
      if (second !== undefined && second.type === 'StringLiteral') {
        const flags = second.value;
        if (isValidRegExpFlags(flags) === false) {
          context.addNodeDiagnostic(second, {
            category: 'lint/noInvalidRegExp',
            message: `Invalid flags supplied to RegExp constructor ${flags}`,
          });
        }
      }
    }

    return node;
  },
};
