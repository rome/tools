/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  VariableDeclarationStatement,
  variableDeclarationStatement,
  AnyNode,
} from '@romejs/js-ast';

export default function VariableDeclarationStatement(node: AnyNode) {
  node = variableDeclarationStatement.assert(node);
  throw new Error('unimplemented');
}
