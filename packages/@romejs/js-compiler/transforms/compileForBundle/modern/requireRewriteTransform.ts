/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  assignmentIdentifier,
  assignmentExpression,
  identifier,
} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {doesNodeMatchPattern, template, inheritLoc} from '@romejs/js-ast-utils';
import {getPrefixedNamespace, getOptions} from '../_utils';

export default {
  name: 'requireRewriteTransform',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    const {relativeSourcesToModuleId, moduleId} = getOptions(context);

    // Replace all references to module.exports to the correct version
    if (node.type === 'MemberExpression' && doesNodeMatchPattern(
      node,
      'module.exports',
    )) {
      return identifier.create({
        name: getPrefixedNamespace(moduleId),
        loc: inheritLoc(node, 'module.exports'),
      });
    }

    // Replace all assignments of module.exports to the correct version
    if (node.type === 'AssignmentExpression' && doesNodeMatchPattern(
      node.left,
      'module.exports',
    )) {
      return assignmentExpression.create({
        operator: node.operator,
        left: assignmentIdentifier.create({
          name: getPrefixedNamespace(moduleId),
          loc: inheritLoc(node, 'module.exports'),
        }),
        right: node.right,
      });
    }

    // Replace import foo = require('module');
    if (node.type === 'TSImportEqualsDeclaration' &&
      node.moduleReference.type === 'TSExternalModuleReference') {
      return (
        template.statement`const ${node.id} = require(${node.moduleReference.expression});`
      );
    }

    // Now handle normal `require('module')`
    if (node.type !== 'CallExpression') {
      return node;
    }

    const {callee} = node;
    if (callee.type !== 'ReferenceIdentifier' || callee.name !== 'require') {
      return node;
    }

    const sourceArg = node.arguments[0];
    if (sourceArg.type !== 'StringLiteral') {
      return node;
    }

    if (path.scope.hasBinding('require')) {
      return node;
    }

    const replacement = relativeSourcesToModuleId[sourceArg.value];
    if (typeof replacement === 'string') {
      return identifier.create({
        name: getPrefixedNamespace(replacement),
      });
    }

    return node;
  },
};
