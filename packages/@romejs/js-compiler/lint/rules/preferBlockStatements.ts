/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CompilerContext, Path} from '@romejs/js-compiler';
import {AnyNode, AnyStatement, blockStatement} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

function addDiagnostic(context: CompilerContext, statement: AnyStatement): void {
  context.addFixableDiagnostic(
    {
      old: statement,
      fixed: blockStatement.quick([statement]),
    },
    descriptions.LINT.PREFER_BLOCK_STATEMENT,
  );
}

export default {
  name: 'preferBlockStatements',
  enter(path: Path): AnyNode {
    const {context, node} = path;

    if (node.type === 'IfStatement') {
      if (node.consequent.type !== 'BlockStatement') {
        addDiagnostic(context, node.consequent);
      }

      if (
        node.alternate !== undefined &&
        node.alternate.type !== 'BlockStatement' &&
        node.alternate.type !== 'IfStatement'
      ) {
        addDiagnostic(context, node.alternate);
      }
    } else if (
      node.type === 'ForStatement' ||
      node.type === 'ForInStatement' ||
      node.type === 'ForOfStatement' ||
      node.type === 'DoWhileStatement' ||
      node.type === 'WhileStatement' ||
      node.type === 'WithStatement'
    ) {
      if (
        node.body.type !== 'BlockStatement' &&
        node.body.type !== 'EmptyStatement'
      ) {
        addDiagnostic(context, node.body);
      }
    }

    return node;
  },
};
