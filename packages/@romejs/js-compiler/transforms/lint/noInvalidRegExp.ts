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
      // Test Pattern
      if (node.arguments[0] && node.arguments[0].type === 'StringLiteral') {
        const pattern = node.arguments[0].value;
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
      if (node.arguments[1] && node.arguments[1].type === 'StringLiteral') {
        const flags = node.arguments[1].value;
        if (isValidRegExpFlags(flags) === false) {
          context.addNodeDiagnostic(node.arguments[1], {
            category: 'lint/noInvalidRegExp',
            message: `Invalid flags supplied to RegExp constructor ${flags}`,
          });
        }
      }
    }

    return node;
  },
};
