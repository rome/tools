/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';
import {doesNodeMatchPattern} from '@romejs/js-ast-utils';

export default {
  name: 'noDirectMutationState',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      node.type === 'ClassDeclaration' &&
      node.meta.type === 'ClassHead' &&
      doesNodeMatchPattern(node.meta.superClass, 'React.Component')
    ) {
      for (let bodyNode of node.meta.body) {
        if (
          (bodyNode.type === 'ClassMethod' || bodyNode.type === 'ClassPrivateMethod') &&
          bodyNode.kind === 'constructor'
        ) {
          for (const bodyBodyNode of bodyNode.body.body) {
            if (
              bodyBodyNode.type === 'ExpressionStatement' &&
              bodyBodyNode.expression.type === 'CallExpression' &&
              bodyBodyNode.expression.callee.type === 'ReferenceIdentifier' &&
              bodyBodyNode.expression.callee.functionDefinition &&
              bodyBodyNode.expression.callee.functionDefinition.head.async &&
              bodyBodyNode.expression.callee.functionDefinition.body.body.find(
                functionBodyElement => (
                  functionBodyElement.type === 'ExpressionStatement' &&
                  functionBodyElement.expression.type === 'AssignmentExpression' &&
                  functionBodyElement.expression.left.type === 'MemberExpression' &&
                  (
                    (
                      functionBodyElement.expression.left.object.type === 'ThisExpression' &&
                      functionBodyElement.expression.left.property.value.type === 'Identifier' &&
                      functionBodyElement.expression.left.property.value.name === 'state'
                    ) ||
                    (
                      functionBodyElement.expression.left.object.type === 'MemberExpression' &&
                      functionBodyElement.expression.left.object.property.value.type === 'Identifier' &&
                      functionBodyElement.expression.left.object.property.value.name === 'state'
                    )
                  )
                )
              )
            ) {
              path.context.addNodeDiagnostic(node, descriptions.LINT.NO_DIRECT_MUTATION_STATE);
            }
          }
        }
        if (
          (bodyNode.type === 'ClassMethod' || bodyNode.type === 'ClassPrivateMethod') &&
          bodyNode.kind !== 'constructor'
        ) {
          for (const bodyBodyNode of bodyNode.body.body) {
            if (
              bodyBodyNode.type === 'ExpressionStatement' &&
              bodyBodyNode.expression.type === 'AssignmentExpression' &&
              bodyBodyNode.expression.left.type === 'MemberExpression' && 
              doesNodeMatchPattern(bodyBodyNode.expression.left, 'state', true)
              ) {
                path.context.addNodeDiagnostic(node, descriptions.LINT.NO_DIRECT_MUTATION_STATE);
            }
          }
        }
      }
    }

    return node;
  },
};
