/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyStatement} from '@romejs/js-ast';
import isDeclaration from './isDeclaration';

export default function isStatement(
  node: undefined | AnyNode,
): node is AnyStatement {
  if (node === undefined) {
    return false;
  }

  if (isDeclaration(node)) {
    return true;
  }

  switch (node.type) {
    case 'BlockStatement':
    case 'BreakStatement':
    case 'ContinueStatement':
    case 'DebuggerStatement':
    case 'DoWhileStatement':
    case 'EmptyStatement':
    case 'ExpressionStatement':
    case 'ForInStatement':
    case 'ForStatement':
    case 'IfStatement':
    case 'LabeledStatement':
    case 'ReturnStatement':
    case 'SwitchStatement':
    case 'ThrowStatement':
    case 'TryStatement':
    case 'WhileStatement':
    case 'WithStatement':
    case 'ForOfStatement':
      const statement: AnyStatement = node;
      statement;
      return true;

    default:
      // Assert that all statements were handled
      const notStatement: Exclude<AnyNode, AnyStatement> = node;
      notStatement;
      return false;
  }
}
