/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRegExpExpression, AnyRegExpBodyItem} from '@romejs/js-ast';
import {Path, TransformExitResult} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

function isQuantifiedMinZero(el: AnyRegExpBodyItem): boolean {
  return el.type === 'RegExpQuantified' && el.min === 0;
}

function lintEmptyMatches(expr: AnyRegExpExpression): boolean {
  if (expr.type === 'RegExpSubExpression') {
    return expr.body
      .map((item) => {
        if (item.type === 'RegExpGroupNonCapture' || item.type === 'RegExpGroupCapture') {
          return lintEmptyMatches(item.expression);
        } else {
          return isQuantifiedMinZero(item);
        }
      })
      .every(el => el === true);
  } else {
    return lintEmptyMatches(expr.left) || lintEmptyMatches(expr.right);
  }
}

export default {
  name: 'emptyMatches',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;
    if (node.type === 'RegExpLiteral' && lintEmptyMatches(node.expression)) {
      context.addNodeDiagnostic(node, descriptions.LINT.EMPTY_MATCHES);
    }
    return node;
  },
};
