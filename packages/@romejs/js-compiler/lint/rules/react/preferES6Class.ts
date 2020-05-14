/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'preferES6Class',
  enter(path: Path): TransformExitResult {
    const {node} = path;

    if (
      node.type === 'CallExpression' &&
      node.callee.type === 'ReferenceIdentifier' &&
      node.callee.name === 'createReactClass'
    ) {
    path.context.addNodeDiagnostic(
      node,
      descriptions.LINT.PREFER_ES6_CLASS,
    );
   }

   return node;
 },
};
