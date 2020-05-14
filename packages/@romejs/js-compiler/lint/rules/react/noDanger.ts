/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast'
import {descriptions} from '@romejs/diagnostics';

function jsxDanger(node: AnyNode) {
  if (node.type !== 'JSXElement') {
    return false;
  }
  const hasDangerAttribute = !!node.attributes.find((attribute) =>
    attribute.type === 'JSXAttribute' &&
    attribute.name.name === 'dangerouslySetInnerHTML'
  )
  return hasDangerAttribute
}

export default {
  name: 'noDanger',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (jsxDanger(node)) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.NO_DANGER,
      );
    }

    return node;
 },
};
