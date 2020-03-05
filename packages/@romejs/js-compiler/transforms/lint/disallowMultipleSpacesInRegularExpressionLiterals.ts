/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

const multipleSpacesPattern = /( {2,})(?: [+*{?]|[^+*{?]|$)/gu;

export default {
  name: 'disallowMultipleSpacesInRegularExpressionLiterals',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (
      (node.type === 'NewExpression' &&
        node.callee.type === 'ReferenceIdentifier' &&
        node.callee.name === 'RegExp' &&
        node.arguments.length > 0 &&
        node.arguments[0].type === 'StringLiteral' &&
        multipleSpacesPattern.test(node.arguments[0].value)) ||
      (node.type === 'RegExpLiteral' &&
        multipleSpacesPattern.test(node.pattern))
    ) {
      context.addNodeDiagnostic(node, {
        category: 'lint/disallowMultipleSpacesInRegularExpressionLiterals',
        message: 'Disallow multiple spaces in regular expression literals',
      });
    }

    return node;
  },
};
