/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyVariableIdentifier} from '@romejs/js-ast';

export default function isVariableIdentifier(
  node: AnyNode,
): node is AnyVariableIdentifier {
  return node.type === 'BindingIdentifier' || node.type ===
  'AssignmentIdentifier' || node.type === 'ReferenceIdentifier' || node.type ===
  'JSXReferenceIdentifier';
}
