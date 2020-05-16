/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from '@romejs/diagnostics';
import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

function jsxIframeMissingTitle(node: AnyNode) {
  return (
    node.type === 'JSXElement' &&
    node.name.type === 'JSXIdentifier' &&
    node.name.name === 'iframe' &&
    !node.attributes.some((attribute) =>
      attribute.type === 'JSXAttribute' &&
      attribute.name.name === 'title' &&
      attribute.value &&
      ((attribute.value.type === 'StringLiteral' &&
      attribute.value.value.length > 0) ||
      (attribute.value.type === 'JSXExpressionContainer' &&
      ((attribute.value.expression.type === 'ReferenceIdentifier' &&
      attribute.value.expression.name !== 'undefined') ||
      (attribute.value.expression.type === 'StringLiteral' &&
      attribute.value.expression.value.length > 0))))
    )
  );
}

export default {
  name: 'jsxA11yIframeHasTitle',

  enter(path: Path): AnyNode {
    const {node} = path;

    if (jsxIframeMissingTitle(node)) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.REACT_JSX_A11Y_IFRAME_HAS_TITLE,
      );
    }

    return node;
  },
};
