/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyStatement} from '@romejs/js-ast';

export default function isStatement(
  node: undefined | AnyNode,
): node is AnyStatement {
  if (node === undefined) {
    return false;
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
    case 'FunctionDeclaration':
    case 'IfStatement':
    case 'LabeledStatement':
    case 'ReturnStatement':
    case 'SwitchStatement':
    case 'ThrowStatement':
    case 'TryStatement':
    case 'VariableDeclaration':
    case 'WhileStatement':
    case 'WithStatement':
    case 'ClassDeclaration':
    case 'ExportAllDeclaration':
    case 'ExportDefaultDeclaration':
    case ' ExportLocalDeclaration':
    case 'ForOfStatement':
    case 'ImportDeclaration':
    case 'FlowDeclareClass':
    case 'FlowDeclareFunction':
    case 'FlowDeclareInterface':
    case 'FlowDeclareModule':
    case 'FlowDeclareModuleExports':
    case 'FlowDeclareOpaqueType':
    case 'FlowDeclareVariable':
    case 'FlowInterfaceDeclaration':
    case 'TypeAliasTypeAnnotation':
    case 'FlowOpaqueType':
    case 'TypeAliasTypeAnnotation':
      return true;

    default:
      return false;
  }
}
