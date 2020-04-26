/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Scope} from '@romejs/js-compiler';

export default function hasPotentialSideEffects(
  node: undefined | AnyNode,
  scope: Scope,
): boolean {
  if (node === undefined) {
    return false;
  }

  switch (node.type) {
    case 'ExportLocalDeclaration':
      if (node.declaration === undefined) {
        return false;
      } else {
        return hasPotentialSideEffects(node.declaration, scope);
      }

    case 'ExportExternalDeclaration':
      return true;

    case 'FunctionExpression':
    case 'FunctionDeclaration':
      return false;

    case 'ClassDeclaration':
      return (
        node.meta.superClass !== undefined ||
        !hasPotentialSideEffects(node.meta.superClass, scope)
      );

    case 'ReferenceIdentifier':
      // Variables that aren't in scope and aren't registered globals could trigger a getter
      // Unlikely but let's aim for 100% correctness
      return (
        scope.getRootScope().isGlobal(node.name) || scope.hasBinding(node.name)
      );

    case 'VariableDeclaration': {
      for (const declarator of node.declarations) {
        if (hasPotentialSideEffects(declarator, scope)) {
          return true;
        }
      }
      return false;
    }

    case 'VariableDeclarator':
      return (
        hasPotentialSideEffects(node.id, scope) ||
        hasPotentialSideEffects(node.init, scope)
      );

    case 'SpreadProperty':
    case 'SpreadElement':
      return hasPotentialSideEffects(node.argument, scope);

    case 'BindingAssignmentPattern':
      return hasPotentialSideEffects(node.right, scope);

    case 'ObjectExpression':
    case 'BindingObjectPattern': {
      for (const prop of node.properties) {
        if (hasPotentialSideEffects(prop, scope)) {
          return true;
        }
      }
      return false;
    }

    case 'StaticPropertyKey':
      return false;

    case 'ComputedPropertyKey':
      return hasPotentialSideEffects(node.value, scope);

    case 'BindingObjectPatternProperty':
    case 'ObjectProperty':
      return (
        hasPotentialSideEffects(node.key, scope) ||
        hasPotentialSideEffects(node.value, scope)
      );

    case 'BindingArrayPattern':
    case 'ArrayExpression': {
      for (const elem of node.elements) {
        if (hasPotentialSideEffects(elem, scope)) {
          return true;
        }
      }
      return false;
    }

    case 'StringLiteral':
    case 'NumericLiteral':
    case 'BooleanLiteral':
    case 'NullLiteral':
      return false;
  }

  return true;
}
