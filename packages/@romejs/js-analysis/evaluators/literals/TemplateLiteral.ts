/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {AnyNode, TemplateLiteral, templateLiteral} from '@romejs/js-ast';
import ExhaustiveT from '../../types/ExhaustiveT';
import StringT from '../../types/StringT';

export default function TemplateLiteral(node: AnyNode, scope: Scope) {
  node = templateLiteral.assert(node);
  for (const expr of node.expressions) {
    new ExhaustiveT(
      scope,
      expr,
      scope.evaluate(expr),
      new StringT(scope, undefined),
    );
  }
  return new StringT(scope, node);
}
