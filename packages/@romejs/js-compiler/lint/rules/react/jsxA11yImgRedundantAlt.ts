/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from '@romejs/diagnostics';
import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

function jsxImgRedundantAlt(node: AnyNode) {
  return (
    node.type === 'JSXElement' &&
    node.name.type === 'JSXIdentifier' &&
    node.name.name === 'img' &&
    node.attributes.some((attribute) =>
      attribute.type === 'JSXAttribute' &&
      attribute.name.name === 'alt' &&
      attribute.value &&
      attribute.value.type === 'StringLiteral' &&
      /(image)|(picture)|(photo)/i.test(attribute.value.value)
    )
  );
}

export default {
  name: 'jsxA11yImgRedundantAlt',

  enter(path: Path): AnyNode {
    const {node} = path;

    if (jsxImgRedundantAlt(node)) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.REACT_JSX_A11Y_IMG_REDUNDANT_ALT,
      );
    }

    return node;
  },
};
