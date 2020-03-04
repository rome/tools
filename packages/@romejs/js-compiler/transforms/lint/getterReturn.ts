/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, ObjectMethod} from '@romejs/js-ast';

export default {
  name: 'getterReturn',
  enter(path: Path): AnyNode {
    const {node} = path;
    switch (node.type) {
      case 'ObjectExpression':
        if (
          node.properties.length !== 0 &&
          (node.properties.find(
            node => node.type === 'ObjectMethod' && node.kind === 'get',
          ) as ObjectMethod) &&
          (node.properties.find(
            node => node.type === 'ObjectMethod' && node.kind === 'get',
          ) as ObjectMethod).body &&
          (node.properties.find(
            node => node.type === 'ObjectMethod' && node.kind === 'get',
          ) as ObjectMethod).body.body &&
          (node.properties.find(
            node => node.type === 'ObjectMethod' && node.kind === 'get',
          ) as ObjectMethod).body.body.find(
            node => node.type === 'ReturnStatement',
          ) === undefined
        ) {
          path.context.addNodeDiagnostic(node, {
            category: 'lint/getterReturn',
            message: 'Class get method does not have a return statement',
          });
        }
        break;
      case 'ObjectProperty':
        if (
          node.value &&
          node.value.type === 'FunctionExpression' &&
          node.value.body &&
          node.value.body.body &&
          node.value.body.body.find(node => node.type === 'ReturnStatement') ===
            undefined
        ) {
          path.context.addNodeDiagnostic(node, {
            category: 'lint/getterReturn',
            message: 'Class get method does not have a return statement',
          });
        }
        break;
      case 'ClassDeclaration':
        if (
          node.meta.body[0] &&
          node.meta.body[0].type === 'ClassMethod' &&
          node.meta.body[0].kind === 'get' &&
          node.meta.body[0].body.body.filter(
            node => node.type === 'ReturnStatement',
          ).length === 0
        ) {
          path.context.addNodeDiagnostic(node, {
            category: 'lint/getterReturn',
            message: 'Class get method does not have a return statement',
          });
        }
        break;
      default:
        break;
    }
    return node;
  },
};
