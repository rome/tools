/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRegExpBodyItem} from '@romejs/js-ast';
import {Path, TransformExitResult} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

function isQuantifiedMinZero(el: AnyRegExpBodyItem): boolean {
  return el.type === 'RegExpQuantified' && el.min === 0;
}

export default {
  name: 'noEmptyCharacterClass',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;
    if (
      node.type === 'RegExpLiteral' &&
      node.expression.type === 'RegExpSubExpression'
    ) {
      const {body} = node.expression;
      if (body && body.every(isQuantifiedMinZero)) {
        context.addNodeDiagnostic(node, descriptions.LINT.EMPTY_MATCHES);
      }
    }
    return node;
  },
};
