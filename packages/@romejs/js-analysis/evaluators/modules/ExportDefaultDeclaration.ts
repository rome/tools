/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  ExportDefaultDeclaration,
  exportDefaultDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import Hub from '../../Hub';

export default function ExportDefaultDeclaration(
  node: AnyNode,
  scope: Scope,
  {evaluator}: Hub,
) {
  node = exportDefaultDeclaration.assert(node);

  const decl = node.declaration;
  const declType = scope.evaluate(decl);
  evaluator.addExport('default', declType);
  return declType;
}
