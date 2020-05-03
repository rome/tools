/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, JSXAttribute} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noChildrenProp',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      (
        node.type === 'JSXElement' &&
        node.attributes.find(attribute => (attribute as JSXAttribute).name.name === 'children')
      ) || (
        node.type === 'CallExpression' &&
        node.callee.type === 'MemberExpression' &&
        node.callee.object.type === 'ReferenceIdentifier' &&
        node.callee.object.name === 'React' && 
        node.callee.property.value.type === 'Identifier' &&
        node.callee.property.value.name === 'createElement' &&
        node.arguments[1].type === 'ObjectExpression' &&
        node.arguments[1].properties.find(property => (
          property.type === 'ObjectProperty' &&
          property.key.value.type === 'Identifier' &&
          property.key.value.name === 'children'
        ))
      )
    ) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.NO_CHILDREN_PROP,
      );
    }

    return node;
  },
};
