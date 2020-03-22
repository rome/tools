/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyIdentifier} from '@romejs/js-ast';

export default function isIdentifierish(node: AnyNode): node is AnyIdentifier {
  return node.type === 'Identifier' || node.type === 'JSXIdentifier' ||
  node.type === 'JSXReferenceIdentifier' || node.type === 'BindingIdentifier' ||
  node.type === 'AssignmentIdentifier' || node.type === 'ReferenceIdentifier';
}
